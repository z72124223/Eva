# EVA Canonical Workspace Tree

> 此為本專案唯一結構來源
> **完整規範請見 [ARCHITECTURE_RULES.md](./ARCHITECTURE_RULES.md)**  
> 本檔僅列純目錄結構以供工具比對。

---

> 本檔僅列出 **純粹的目錄/檔案結構**，供工具對照與維護。
> 所有「設計補充」與「成果紀錄」已移至 `ARCHITECTURE/ARCHIVE/` 下的相關文件。

```tree # canonical
├── Cargo.toml
├── Cargo.lock
├── .env
├── .github/
├── ARCHITECTURE/
│   ├── AUTO_USE.md
│   ├── PROGRESS_CHECKLIST.md
│   └── ARCHITECTURE_TREE_CANONICAL.md  ← (this file)
├── crates/
│   ├── eva-knowledge-base/
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── eva-system-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── schema.rs       # 資料結構定義
│   │       ├── db.rs          # SQLite 資料庫操作
│   │       ├── scanner.rs     # 專案掃描與異動偵測
│   │       └── query.rs       # 自省資料查詢與統計
│   ├── eva-tools/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── tui_dashboard.rs
│   └── task_planner/
│       ├── Cargo.toml
│       └── src/
├── src/
│   ├── api_server.rs
│   ├── auto_scheduler.rs
│   ├── boredom_detector.rs
│   ├── scene_scoring.rs
│   ├── app/
│   ├── ui/
│   ├── services/
│   ├── tools/
│   ├── state/
│   ├── knowledge_base/
│   └── …
├── tools/
├── ui/
└── …
```

---

## [2025Q2] AI+代理 MVP 架構進化設計

- [強制] 每次合併、功能變動，必須自動執行 CI、完整自動化測試、依賴與 feature flag 一致性檢查。
- [強制] 架構與模組規則（llm_router、run_with_args、AI 輸入框、TUI/CLI 雙軌、安全權限）皆為必須嚴格落實，不得違反。
- 新增 llm_router 代理層：負責將 AI 輸入語意解析並調度對應功能 API。
- 各功能模組（如升級、自省、task_planner、知識庫）統一暴露 run_with_args 介面，供代理層調用。
- 主控 AI 輸入框取代傳統選單，支援自然語言指令，AI 能自動判斷意圖並執行。
- 保留 TUI/CLI 傳統操作模式，允許用戶雙軌切換。
- 所有代理調用均有日誌與權限控管，確保安全。

### [2025Q2] 本地長期記憶設計要點
- [強制] 本地記憶規則（DB schema、API、查詢/編輯/刪除、權限、個資主權、與其他系統串接）皆為必須嚴格落實，不得違反。
- 建立本地記憶資料庫（用戶偏好、對話紀錄、操作歷史、專案狀態…）。
- 代理層調用 LLM 前自動查詢記憶，將關聯內容注入 prompt/context。
- 支援查詢、編輯、刪除個人記憶，完全自主。
- 與知識庫、日誌、任務系統深度串接，實現個人化 AI。

### [2025Q2] 本地長期記憶 DB schema、API 草案與串接流程圖

- [強制] DB schema、API 草案、串接流程圖、資料流、控制流等設計皆為必須嚴格落實，不得違反。

#### Scaffold 實作說明
- memory_store.rs 定義 Memory 結構與 MemoryStore trait，標準化增查改刪 API。
- memory_store_sqlite.rs 實作 SqliteMemoryStore，支援自動建表、條件查詢、JSON 標籤、彈性擴充。
- 代理層可直接呼叫 trait API 實現記憶查詢、寫入與管理。
- 後續可擴充：加密存儲、權限控管、TUI/CLI 查詢、與 LLM prompt/context 串接、記憶型別高階 API。

#### DB schema（資料結構設計）
| id  | user_id | memory_type | content           | created_at          | tags         |
|-----|---------|------------|-------------------|---------------------|--------------|
| 1   | 001     | chat       | 「今天修了 bug」   | 2025-05-08 09:00:00 | ["日誌"]      |
| 2   | 001     | preference | dark_mode=true    | 2025-05-08 09:01:00 | ["設定"]      |
| 3   | 001     | task       | 「明天要升級模組」 | 2025-05-08 09:02:00 | ["任務提醒"]  |

#### API 草案（程式介面設計）
```rust
struct Memory {
    id: i64,
    user_id: String,
    memory_type: String,
    content: String,
    created_at: DateTime<Utc>,
    tags: Vec<String>,
}

trait MemoryStore {
    fn add_memory(&self, mem: Memory) -> Result<()>;
    fn get_memories(&self, filter: MemoryQuery) -> Vec<Memory>;
    fn update_memory(&self, id: i64, new_content: &str) -> Result<()>;
    fn delete_memory(&self, id: i64) -> Result<()>;
}
```

#### 串接流程圖（資料流/控制流設計）
```
[用戶輸入] → [AI 代理層]
    ↓
[查詢本地記憶 DB]
    ↓
[將相關記憶內容注入 LLM prompt]
    ↓
[LLM 產生回應/決策]
    ↓
[代理層執行功能/記錄新記憶]
    ↓
[結果回饋用戶 & 寫入新記憶]
```
---

## 執行規劃與變更管理流程

1. **近期目標與里程碑**：
   - 每季（或專案階段）明確設定目標（如自動化驗證、錯誤修復、知識串接等），並公告於專案文件。
2. **變更提案（RFC）流程**：
   - 任何架構、流程、驗證機制變動，必須先提出 RFC（Request For Comments）或設計提案，明確標註依據的藍圖條文。
   - 提案需經窗口（或專責審查小組）討論、審查，並記錄於設計歷史。
3. **自動化驗證與 CI/CD 強化**：
   - 所有合併請求、功能變動，必須自動執行單元測試、集成測試、架構規則檢查。
   - 驗證流程以減少人工負擔為最高原則。
4. **知識回饋與設計演進紀錄**：
   - 每次設計、驗證、修復經驗，皆記錄於設計歷史檔案，定期回顧優化。
5. **分工與窗口責任**：
   - 明確指定每一模組、流程的負責窗口，所有窗口必須熟悉藍圖規則，並負責教育新成員。
6. **會議與共識**：
   - 定期召開架構共識會議，針對藍圖、規則與近期目標進行確認。

---

若需查看過去的設計演進、測試紀錄、或其他說明，請前往：

* `ARCHITECTURE/ARCHIVE/ARCHITECTURE_TREE_DESIGN_HISTORY.md`
