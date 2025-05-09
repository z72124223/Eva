# EVA 下一階段（D7 ~ D9）里程碑

讓 EVA 真正「學會」並「持續進化」

| 里程碑 | 目標 | 主要產出 | 完成標誌 |
|--------|------|----------|----------|
| D7‑1 | 知識庫引擎 (eva-knowledge-base crate) | 讀/寫 API + Query DSL | kb::query("unresolved import") 回傳歷史修補 |
| D7‑2 | Prompt 增強器：產生 LLM prompt 前自動插入 KB 條目 | prompt_adapter.rs 讀 KB → 注入「Avoid cross‑crate use」等提示 | 讓 LLM patch 命中率提升，重試次數顯著下降 |
| D7‑3 | 本地 LLM 微調腳本：用 error‑patch 對自行 fine‑tune | scripts/finetune.sh、ft_config.json | sandbox 上跑完 1 epoch，loss 下降 |
| D8‑1 | Runtime 監控鉤子：把 執行期 panic/log 也送進 KB | panic_hook.rs 插入 record_error() | 故意觸發 panic → KB 新增 runtime 錯誤 |
| D8‑2 | 背景 Watcher：auto_loop_executor 守護模式<br>‑ file‑watch + cron fine‑tune | CLI：eva-tools auto-loop --watch | 檔案變更後自動重跑閉環；每天 03:00 自動 fine‑tune |
| D9‑1 | CI / 防呆整合：<br>‑ GitHub Action：cargo check && cargo test<br>‑ 失敗時自動開 PR 並附 patch | .github/workflows/ci.yml + ci_reporter.rs | 在 PR 上可看到「EVA 修補建議 △▢✓」 |
| D9‑2 | 人機審核 Dashboard (可選)：簡易 TUI 或 Web，列出 fix_history/ | tui_dashboard.rs 或 web_dashboard/ | 能瀏覽 patch、點擊 accept/reject |


---

### Definition of Done（驗收指標）

- **D7-1 知識庫引擎**
  - 通過 API 單元測試數 ≥ 10
  - Query DSL 支援 3 種以上查詢型態
  - 1 篇 KB 查詢教學文檔 PR
  - Demo: 執行 kb::query("unresolved import") 可回傳紀錄

- **D7-2 Prompt 增強器**
  - Prompt patch 命中率提升 ≥ 20%
  - 3 種以上 KB 條目自動注入
  - 通過 prompt_adapter 單元測試 ≥ 5
  - Demo: LLM patch 重試次數下降有數據對比

- **D7-3 本地 LLM 微調腳本**
  - fine-tune 腳本可於 sandbox 跑 1 epoch
  - loss 下降超過 10%
  - ft_config.json 配置可重現
  - Demo: loss 曲線圖

- **D8-1 Runtime 監控鉤子**
  - 故意 panic 可於 KB 查到紀錄
  - panic_hook 單元測試 ≥ 3
  - 監控鉤子文檔 PR

- **D8-2 背景 Watcher**
  - auto-loop --watch 可監控檔案變更自動重跑
  - cron fine-tune 可自動執行並產生 log
  - Demo: watcher 執行 log 截圖

- **D9-1 CI / 防呆整合**
  - GitHub Action 100% 覆蓋所有主要路徑
  - PR 自動附 patch 並可人工審核
  - ci_reporter.rs 單元測試 ≥ 5
  - Demo: PR 上顯示 EVA 修補建議

- **D9-2 人機審核 Dashboard**
  - TUI/Web 可瀏覽 patch 並點擊 accept/reject
  - fix_history/ 檔案自動同步
  - Demo: dashboard 操作錄影

> 本文件由 EVA 輔助自動生成，請依據實際進度補充與修正。
