# 自動插入 use 功能設計（待驗證）

- 當偵測到 unresolved import `xxx` 時：
    1. 解析錯誤訊息取得 import 路徑。
    2. 找到出錯來源檔案（如 main.rs）。
    3. 若檔案尚未 use xxx，則在檔頭第一個非註解/非空行後插入 `use xxx;`。
    4. 寫入修正後的檔案內容。
    5. 呼叫 record_error 寫入修補紀錄。
- 若自動插入失敗，記錄 need-manual。

- error_recovery::handle 會自動偵測 cargo check 的錯誤訊息：
    - 若遇到 can't find crate for `xxx`，會自動補 xxx = "*" 到 Cargo.toml [dependencies]（若不存在時）。
    - 若遇到 unresolved import `xxx`，會提示並記錄到 error_log.md，但目前僅提示，未自動插入 use（避免誤傷）。
- 若 auto-fix 仍無法自救，auto_loop_executor 會自動進入標記分支流程。

- auto_loop_executor.rs 會自動執行 cargo check，遇錯誤自動修復，最多重試 MAX 次，失敗則自動建立 git branch（分支名稱格式如 eva-auto-review-YYYYMMDDHHMMSS）供人工複查。
- tag_for_review() 會呼叫 git checkout -b 建立新分支，並於終端提示。
