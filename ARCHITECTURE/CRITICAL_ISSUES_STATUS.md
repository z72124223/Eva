# 二、仍待處理的關鍵問題 ⚠️ (2025-05-08)

| # | 問題 | 影響 | 最新狀態 |
|---|------|------|-----------|
| WP‑11 | WebSocket Listener/Intent Channel | Intent 多源協作、WS 連線、Prometheus 指標 | 🟢 Done（2025-05-09） |
| WP-12 | RFC: 統一 Message Bus | 🟡 In Progress | rfc/message-bus | EVA 自穩定設計藍圖（4週 Sprint） | 🟡 In Progress（詳見 SELF_STABLE_DESIGN.md） |

### WP‑12 詳細進度
1. 基礎層：eva_kernel 設計與實現（週 1）
2. 決策層：Design-doc 流程（週 2）
3. 技術層：Message Bus 與驗證（週 3）
4. 介面層：統一 API 格式（週 4）

每個階段完成後需提交 PR 並通過 CI 測試。
| 1 | CI 覆蓋率門檻仍寫 70 % (CI_PR_AUTOMATION.md) | 自動修補機制風險 ≫70 %的測試強度 | **✅ 已調整** → `eva-tools ≥95 %`, 其他 crate ≥80 % (ci.yml & CI_PR_AUTOMATION.md 更新) |
| 2 | Router 拆分尚未落地（僅列進度表） | 仍是單點失效 + 循環依賴隱憂 | ⏳ 未開始 |
| 3 | Spec 條款 ↔ 真實程式碼自動對映缺失 | 文件漂移風險仍高 | 🔧 已有 `cargo xtask check-layout` POC，*尚未接入 CI* |
| 4 | 資料隱私細節不足 | 加解密／刪除權／角色隔離尚無實作方案 | ❌ 缺失 |
| 5 | README 連結失准：指向 AUTO_FIX.md，實際檔不存在 | 文件可信度下降，CI 產生死鏈 | **✅ 已修正** → 建立 `ARCHITECTURE/AUTO_FIX.md`，連結恢復正常 |
| 6 | 測試失敗熔斷 & 回滾策略未見設計文／程式碼 | 自動修補若連續失敗會瘋狂重試 | ❌ 缺失 |
| 7 | Mutation testing 未引入 | 難以驗證測試對錯誤分支的保護力 | **✅ 已引入** → `ci.yml` 新增 `cargo mutants` 步驟，`--deny-detected` 強制檢測 |
| 8 | Ops 監控 / 降級機制仍缺 | 服務上線後 SLO、告警、降級策略未落地 | ❌ 缺失 |

> 本表格每日自動生成/更新，供監督即時掌握本地檔案與實作進度。

---

## 生成依據
- 腳本掃描檔案：`.github/workflows/ci.yml`, `ARCHITECTURE/CI_PR_AUTOMATION.md`, `xtask/`, `src/**/*.rs` …
- 判定規則：
  - **已調整 / 已引入**：對應程式碼或文件已落地並通過 CI。
  - **POC**：已有原型，但尚未納入正式流程。
  - **缺失**：完全尚未實作。

---

如需手動覆寫或補充說明，請直接編輯本檔或提出 PR。
