# RFC: 統一 Message Bus 與 JSON Schema
*編號*: 2025-05-message-bus
*作者*: EVA-Bot
*狀態*: Draft
*日期*: 2025-05-09

## 1. 背景與動機
目前 WebSocket Listener 直接把純文字 intent 傳入 `EvaCore`，
但未來需要支援：
- 本地 LLM / 雲端 LLM / 第三方 REST API
- 多種訊息型別（intent、plan、memory、patch  …）
為避免各模組各說各話，需建立**統一的 Message Bus 格式**。

## 2. 目標
1. 所有訊息都包成 JSON，並附 `role`、`type`、`payload`、`trace_id`
2. 在入口（WS / REST）驗證 JSON Schema，不合法直接回 422
3. Prometheus 加 Counter：`eva_bus_messages_total{type="intent"}` …
4. 不影響既有 CLI 與 WS 使用流程

## 3. 設計方案
### 3.1 JSON 結構
```jsonc
{
  "role": "user | system | tool",
  "type": "intent | plan | memory | patch",
  "trace_id": "uuid-v4",
  "payload": { /* 依 type 不同有不同 schema */ }
}
```

### 3.2 模組改動
| 模組 | 變動 | 為何需要 |
|------|------|-----------|
| listener/ws.rs | 收到文字 → 轉成 {role:"user",type:"intent",…} | 統一入口格式 |
| planner | 接 intent JSON，回傳 plan JSON | 解耦解析/執行 |
| executor | 根據 plan.payload 決定呼叫哪個 adapter | 統一決策邏輯 |

### 3.3 JSON Schema 驗證
使用 `jsonschema` crate

Schema 檔案結構：
```
schema/
├── base.json        # 基本結構
├── intent.json      # intent 具體格式
├── plan.json        # plan 具體格式
└── patch.json       # patch 具體格式
```

### 3.4 Schema 範例
```jsonc
// schema/base.json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["role", "type", "trace_id", "payload"],
  "properties": {
    "role": { "enum": ["user", "system", "tool"] },
    "type": { "enum": ["intent", "plan", "memory", "patch"] },
    "trace_id": { "type": "string", "format": "uuid" },
    "payload": { "type": "object" }
  }
}

// schema/intent.json
{
  "allOf": [
    { "$ref": "base.json" },
    {
      "if": { "properties": { "type": { "const": "intent" } } },
      "then": {
        "properties": {
          "payload": {
            "required": ["text"],
            "properties": {
              "text": { "type": "string" }
            }
          }
        }
      }
    }
  ]
}
```

## 4. 替代方案
1. 繼續傳純文字
   - 优点：简单
   - 缺点：解析邏輯散落各處
   - **不採用**：維護成本高

2. 使用 gRPC / Protobuf
   - 优点：型別安全
   - 缺点：學習曲線陡峭
   - **不採用**：當前需求不值得

## 5. 相容性影響
- 舊 CLI stdin 仍能用
  - core 自動包成 intent JSON
  - 不影響現有使用者

- 其餘模組無破壞性改動
  - 向下相容
  - 測試覆蓋率高

## 6. 安全與風險
1. Schema 驗證
   - 不合法立即拒絕
   - 防止任意欄位注入

2. trace_id
   - 分散式追蹤
   - 診斷問題方便

## 7. 實作計畫
| 週次 | 工作 | 產出 |
|------|------|------|
| W1 | schema/*.json + listener 改動 | PR #xxx |
| W2 | planner / executor 改接 Bus | PR #yyy |
| W3 | Prom metrics / Contract test | PR #zzz |

## 8. 驗收條件 (DoD)
- cargo test 全綠
- wscat 發 {…} JSON → core log 收到 trace_id
- /metrics 顯示 eva_bus_messages_total ≥1
- CI snapshot 測試通過

## 9. 附件 / 參考
- Issue #123: Message Bus discussion
- JSON Schema 雏形
- Prometheus 指標設計文

### 3.2 Schema 驗證
```jsonc
// base schema
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["role", "type", "trace_id", "payload"],
  "properties": {
    "role": { "enum": ["user", "system", "tool"] },
    "type": { "enum": ["intent", "plan", "memory", "patch"] },
    "trace_id": { "type": "string", "format": "uuid" },
    "payload": { "type": "object" }
  }
}

// intent-specific schema
{
  "allOf": [
    { "$ref": "base.json" },
    {
      "if": { "properties": { "type": { "const": "intent" } } },
      "then": {
        "properties": {
          "payload": {
            "required": ["text"],
            "properties": {
              "text": { "type": "string" }
            }
          }
        }
      }
    }
  ]
}
```

### 3.3 實作步驟
1. 建立 `eva-core/src/bus/schema/` 放置所有 JSON Schema
2. 實作 `MessageValidator` 在入口驗證
3. 修改 WebSocket Handler 使用新格式
4. 添加 Prometheus 指標

### 3.4 遷移策略
1. 先保留舊格式，新格式為可選
2. 測試穩定後，逐步移除舊格式
3. 確保所有測試通過

## 4. 風險評估
1. 向下相容性風險：需要同時支援新舊格式
2. 性能影響：增加 Schema 驗證
3. 錯誤處理：需要明確的錯誤訊息

## 5. 可選方案
1. 不使用 JSON Schema，改用 Rust 的 derive
2. 使用 Protobuf 而不是 JSON
3. 使用其他驗證機制

## 6. 實作時間表
1. 設計與討論：1 天
2. 實作 Schema：1 天
3. 實作驗證器：1 天
4. 測試與調整：2 天
5. 遷移現有代碼：2 天

## 7. 附錄
### 7.1 相關資源
- [JSON Schema Draft 7](https://json-schema.org/draft-07/schema)
- [Prometheus best practices](https://prometheus.io/docs/practices/)
- [WebSocket RFC 6455](https://tools.ietf.org/html/rfc6455)

### 7.2 關鍵指標
- `eva_bus_messages_total{type="intent"}`
- `eva_bus_validation_errors_total`
- `eva_bus_schema_version`

<!-- Minor whitespace fix -->
