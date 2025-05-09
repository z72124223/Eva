# EVA 專案進度追蹤清單

## 正在進行／尚未完成重點項目（依優先級排序）

| 編號 | 項目 | 優先級 | 狀態 | 建議動作 |
|------|------|--------|------|---------|
| 0 | 進度同步自動化／知識回饋機制強化 | ★★★★☆ | ⏳ 進行中 | 建立自動同步進度與知識回饋流程 | 
| 1 | AI+代理 MVP 架構改造（AI 驅動主控 + 代理層） | ★★★★★ | ⏳ 進行中 | 規劃與設計中 |
| 2 | 意圖-功能對應表設計（升級/自省/任務/KB查詢） | ★★★★☆ | ⏳ 進行中 | 列 MVP 功能清單 |
| 3 | llm_router.rs 擴充（語意解析+調度） | ★★★★☆ | 未開始 | 設計代理層 API |
| 4 | 各功能 API 化與統一介面 | ★★★☆☆ | 未開始 | 包裝現有模組 |
| 5 | TUI/CLI AI 指令入口 | ★★★☆☆ | 未開始 | 單一輸入框串接 |
| 6 | 測試與安全控制 | ★★★☆☆ | 未開始 | 權限、日誌、回退 |
| 7 | 本地長期記憶設計與串接 LLM | ★★★★★ | ⏳ 進行中 | 設計記憶結構、API 串接 |
| 8 | Scaffold 實作：memory_store.rs/SQLite | ★★★★★ | ✅ 已完成 | 已具備增查改刪 API，可供代理層/AI調用 |
| 9 | 串接代理層與 LLM context | ★★★★☆ | 未開始 | 將記憶查詢/寫入納入 AI prompt/context 流程 |
| 10 | TUI/CLI 記憶查詢介面 | ★★★☆☆ | 未開始 | 提供用戶可查/改/刪記憶的操作 |
| 11 | 加密存儲與權限控管 | ★★★☆☆ | 未開始 | 強化安全性與多用戶隔離 |
---

## 主要開發任務與藍圖里程碑

- [x] 架構規則條文化（ARCHITECTURE_TREE_CANONICAL.md 條列嚴格規則）
- [x] 執行規劃與變更管理流程條文化（藍圖新增規劃與 RFC 流程條文）
- [x] D7-1 知識庫 crate + 改寫 record_error()
- [x] D7-2 提示增強
- [x] D7-3 微調腳本（首輪 demo）
- [x] D8-1 運行時 panic hook
- [x] D8-2 守護守護進程（daemon/watcher/cron）
- [x] D9-1 CI / PR bot（自動修補 PR 流程）
- [x] Finetune artifacts 歸檔／版本化（scripts/finetune.sh 產物與日誌）
- [x] insert_use_stmt 自動寫入 use（AUTO_USE.md 實作）
- [x] 安全參數 YAML 化並載入（MAX_PATCH_SIZE, forbid list）
- [x] README 入口指向 ARCHITECTURE_TREE_CANONICAL.md
- [x] CI 覆蓋率門檻（tarpaulin --fail-under 95 for eva-tools / 80 for others）
- [x] finetune.sh 路徑已參數化（CARGO_MANIFEST_DIR）
- [x] Smoke Test 條款：
    - [x] cargo check && cargo test
    - [x] eva-tools auto-loop --once
- [x] Dashboard 雛形（UI 骨架 + fix_history 裝載機 + 接受/拒絕流程）
- [x] Workspace resolver = "3"、edition 同步（隱藏技術債）
- [x] 各模式 stub 已串聯並可執行
- [x] 系統自省核心（eva-system-core）建立，提供SQL資料庫結構檔案追蹤功能

## 測試與交付
- [ ] Smoke Test 全流程
    - [ ] cargo check & cargo test
    - [ ] auto_loop_executor --once 成功
    - [ ] CI 綠燈
- [ ] 文件補齊（README、藍圖、API 註解）
- [ ] Tags v0.4（Cargo.toml、Git 標籤）

---

## 使用說明
- 已完成的項目會打勾（[x]），未完成項目為空格（[ ]）。
- 可於每次階段性開發/驗收後更新此清單。
- 建議與 ARCHITECTURE_TREE_CANONICAL.md、CI_PR_AUTOMATION.md 一起參考。

---

> 本清單可作為團隊進度會議、交付驗收、或自我管理依據。
