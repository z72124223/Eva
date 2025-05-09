# EVA Memory API 隱私與合規規範

1. 每筆記憶必須標註 `owner_id`（唯一用戶）與 `scope` 欄位：
   - `scope = ai-visible`：可注入 AI prompt
   - `scope = human-only`：僅人類界面可見
   - `scope = private`：僅本人可查詢
2. 所有查詢、刪除、更新 API 必須驗證 owner_id 權限。
3. 提供「資料可攜權」API：用戶可一鍵匯出所有屬於自己的記憶。
4. 提供「被遺忘權」API：用戶可要求徹底刪除所有 owner_id 相關資料，系統應回傳刪除證明。
5. API 必須支援權限 middleware（token/role guard），防止跨用戶查詢。
6. 嚴禁將 scope != ai-visible 的記憶注入 AI prompt，防止機密洩漏。

---

## GDPR/CCPA 合規流程
- 用戶可隨時查詢、匯出、刪除自己的所有記憶。
- 管理員可審計刪除請求並產生紀錄。
- 系統需保證刪除後無法復原。

---

如需實作範例與測試腳本，請參考 `memory_store.rs` 及 `memory_store_sqlite.rs`。
