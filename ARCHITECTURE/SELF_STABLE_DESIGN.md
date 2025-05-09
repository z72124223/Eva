# EVA 自穩定設計藍圖

## 1. 基礎層：極小可控依賴庫

### 目標
- 將 EVA 真正核心邏輯（Intent→Plan→Act、Memory、Patch-safety）收斂到一個 `eva_kernel`
- 周邊功能（UI、TTS、插件）→ 各自獨立 `eva-xyz` crate

### 已完成
- ✅ WebSocket Listener 作為獨立模組
- ✅ Intent channel 實作
- ✅ Prometheus 指標追蹤
- ✅ ACK 回應機制

### 具體做法
- 只暴露 traits/DTO，不暴露具體實作
- 跟外部 crate 關係：全部透過 Cargo.toml path 依賴，版本鎖定到 commit
- 用 adapter/bridge，不准「反向引用」核心

### 為何能解問題
- 依賴樹越小，AI 生成 patch 時就越不會牽動超多 crate
- 可以在 CI 做 `cargo tree -i eva_kernel` → 漂移就紅燈

## 2. 決策層：AI 寫程式的「為什麼」

### 寫程式易，判斷是否該寫才難
- 這件事必須拆成「設計 → 代碼生成 → 驗證」三段

### 階段與內容

#### Design why
- 把需求轉成 RFC / ADR；文本放 docs/rfc/YYYY-mm-dd-xxx.md
- LLM 先產生 diff-based Design Doc（用 markdown code-fence diff），走 reviewer-bot

#### Patch generation
- LLM 依 RFC 產生 patch；每個 patch 只允許改 ≤ N 行 / ≤ M 檔
- 已有的 MAX_PATCH_SIZE.yml + mutation test；超過大小直接拒收

#### Verification
- 單元+整合+mutation test、cargo-deny、cargo-geiger
- 合併前 CI 全部綠；否則自動開 AUTO_FIX

## 3. 技術層：避免「整包重寫」與依賴爆炸

### 分層熔斷

#### patch-safety 層
- 檢查 patch 尺寸、危險關鍵字

#### dependency-safety 層
- ✅ CI 跑 cargo deny & cargo outdated --depth=1，版本漂移就 fail

#### 語義型測試 (contract test)
- 為 eva_kernel 每個 public trait 寫「行為快照」測試
- LLM 改動若破壞行為 → snapshot diff 紅燈

#### 增量版本策略 (x.y.Z)
- 任何 breaking change 只能 bump y，並必須附自動 migration patch
- cargo publish / PR label BREAKING 雙重檢查

## 4. 介面層：統一「API ↔ GPT」對話格式

### 元素與設計

#### 單一 Message Bus
- 全部 Intent / Plan / Patch / Memory，都包成
- `{ "role":"system|user|assistant|patch|memory" }`

#### JSON Schema 驗證
- 在入口 (WS, REST) 先跑 jsonschema → 不合法直接回 422
- GPT 不會給出亂結構 → 減少「亂改」風險

#### Conversation-ID + Trace-ID
- 每個 Bus message 帶 conv_id, trace_id
- 追蹤 Bug 時快速對應 log / metrics

## 🛠️ 落地路線（4 週 Sprint 範例）

### 週 1：建立 eva_kernel／分離 Adapter
- 移動核心 Trait & DTO
- CI 加 cargo tree 漂移 check

### 週 2：Design-doc 流程 + Reviewer-bot
- 新增 docs/rfc/ + hash check
- GitHub Action：design-lint

### 週 3：Message Bus & JSON Schema 驗證
- eva-core::bus 模組 + schema/*.json
- WS/REST 入口改跑 schema

### 週 4：Contract 測試 + Dependency 安全網
- snapshot crate
- cargo deny, cargo outdated Gate

## 完成後

完成後，再把 AI Code-Gen Pipeline（Planner → Patch → Verify → PR）塞進 EVA 自己的循環，就真正做到「能自學、且不會越寫越亂」。
