# EVA 自動錯誤修復流程設計（AUTO_FIX）

> 本文件補足 README 及設計索引中的失效連結，描述 EVA 自動產生修補 PR 的機制與策略。

## 目標
- 減少人工介入：CI 失敗時自動產生可行 patch 並開 PR。
- 確保修補安全：限制修改範圍、需通過測試、覆蓋率不下降。

## 關鍵元件
| 元件 | 路徑 | 說明 |
|------|------|------|
| `eva-tools` | `crates/eva-tools/src/` | 提供 `create-fix-pr` 指令，掃描 CI 日誌、產生 patch 並推送分支。 |
| `auto_loop_executor` | `crates/eva-tools/src/auto_loop_executor.rs` | 循環監控 CI 狀態，觸發自動修補流程。 |
| GitHub Actions | `.github/workflows/ci.yml` | `Auto-fix and create PR if test fails` 步驟呼叫 `create-fix-pr --ci`。 |

## 流程
1. **CI 失敗偵測**：`ci.yml` 設定 `if: failure()` 於最後階段呼叫工具。
2. **產生 patch**：`eva-tools` 解析 `cargo test` 及 linters 的錯誤，嘗試自動插入缺失 `use`、更新 snapshot、套用建議修補程式碼。
3. **建立 PR**：使用 GitHub CLI 建立名為 `auto-fix-YYYYMMDD-HHMMSS` 的分支與 PR，標記 `AUTO_FIX` label。
4. **人為審核**：維持最少 1 Reviewer；若 autopatch 無誤即可 merge。併入後 pipeline 重新執行確認修補成功。

## 安全機制
- **變更範圍限制**：禁止變動 `Cargo.lock` 以外的 lockfile，以免引入惡意 deps。
- **覆蓋率不下降**：合併前再次檢查 tarpaulin 報告，若低於門檻自動加上 `needs-review` 標籤並阻擋 merge。
- **Retry 上限**：同一錯誤類型至多連續修補 3 次；超過則改為開 issue 通知。

## 未來工作
- 整合 `cargo-mutants` 報告，優先修補 mutation 檢測失敗的區塊。
- 觀察 LLM refactor，嘗試引入 AI-based 修補建議。

## [2025/05/08] LLM 整合現場紀錄

### 1. 目錄與模型路徑踩坑
- **原始狀態**：llama/llama-run.exe 及模型放在 `C:/llama/`，EVA 預設路徑也是如此。
- **現場調整**：用戶將 llama 目錄整個搬移到 `C:/Users/use/Desktop/Eva/llama/`（即專案根目錄下）。
- **建議**：若有搬移，請同步修改 EVA 程式內部的模型與執行檔搜尋路徑（建議用相對路徑或專案根目錄下的 llama/）。

### 2. LLM CLI 參數踩坑
- **AI 原本操作**：
  - 起初誤用舊版 llama.cpp 參數（如 `-m`, `-p`）。
  - 修正後改用新版 llama-run.exe 的位置參數格式 `[model_path, prompt]`。
  - 進一步修正為自動加上 `--ngl 999` 強制 GPU。
- **用戶現場操作**：
  - 發現 GPU 未啟用、CPU 占用過高，反饋 CLI 卡死問題。
  - 將 llama 目錄移動到專案根目錄下，並要求所有踩坑與修正細節都記錄於藍圖。

### 3. 溝通與責任分工
- **AI 必須主動修正參數與流程，所有 CLI 呼叫、參數拼接都由 AI 負責，不能推給用戶。**
- **用戶只需回饋現場狀態與需求，AI 應即時修正到完全打通。**

### 4. 建議
- 將 llama 路徑、模型路徑寫成 config 檔或環境變數，避免硬編碼踩坑。
- 現場所有踩坑、修正、溝通紀錄都應同步記錄於本藍圖，方便未來追蹤與新手 onboarding。
