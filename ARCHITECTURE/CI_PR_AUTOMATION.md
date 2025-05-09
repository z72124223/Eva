# EVA 專案 CI & PR 自動化流程藍圖

## 目標
- 當 CI 測試失敗時，自動產生修補 patch，建立修補分支，並自動建立 Pull Request（PR）到 GitHub。
- 減少人工操作，提升修補效率與追蹤透明度。

## 流程概述
1. **CI/CD 檢查**
   - 當 CI 偵測到測試失敗，自動呼叫 `eva-tools create-fix-pr --ci`。
2. **自動修補 PR 流程**
   - 產生 patch 並建立自動修補分支（`auto-review-YYYYMMDDHHMMSS`）。
   - commit 修補內容。
   - push 分支到 GitHub 遠端（origin）。
   - 使用 GitHub CLI（gh）自動建立 Pull Request，PR 內容包含修補說明與知識庫參考。
3. **權限與安全**
   - 使用者需安裝並授權 GitHub CLI（gh）。
   - 需正確設定 git remote 與權限（origin 指向目標 repo）。

## 需求與環境
- 已安裝並登入 gh CLI
- git remote 已設定並有推送權限
- EVA 專案內有 `eva-tools` 工具

## 特色
- 全流程自動化，無需人工干預
- 支援多分支並行修補
- PR 內容自動補充知識庫連結

## 實現檔案
- `.github/workflows/ci.yml`
- `crates/eva-tools/src/main.rs`（create-fix-pr 指令）
- `crates/eva-tools/src/auto_loop_executor.rs`（自動監控與排程）

## 覆蓋率門檻
- CI workflow 已安裝 tarpaulin，並於 `Coverage (eva-tools ≥95%)` 步驟強制 `--fail-under 95`，其餘 crate 在 `Coverage (other crates ≥80%)` 步驟以 `--fail-under 80`，確保低覆蓋率 PR 會自動 fail。

---

> 本藍圖可直接複製到 ARCHITECTURE_TREE_CANONICAL.md 或相關文件，作為 CI & PR 自動化設計說明。
