# EVA 記憶系統可行突破方案（WP-6 擴充版）

| 步驟                | 做法                                                                 | 工時估 | 難度 |
|---------------------|---------------------------------------------------------------------|--------|------|
| a. ContextMemory    | Executor 每完成一步 → memory.log JSONL {trace_id, role, ts, text}   | 0.5 d  | 低   |
| b. Embedding & Vector DB | 選 sentence-transformers all-MiniLM 或 OpenAI embedding；每步寫入向量 DB | 1 d    | 中   |
| c. Retriever        | 在 Planner 前：根據 intent 取 k 條相似記憶，加到 prompt              | 0.5 d  | 中   |
| d. Consolidator Job | nightly Cron：把當日 log 摘要成“episode summary”存長期              | 0.5 d  | 中   |
| e. TTL / Prune      | 向量條目 > M or age > N 天 → 刪或壓縮                                | 0.5 d  | 低   |

## 工作任務
- [ ] 完成 ContextMemory 日誌寫入
- [ ] 整合 embedding 與向量資料庫
- [ ] 實作 Retriever，將相似記憶注入 prompt
- [ ] 開發記憶整理與摘要 job
- [ ] 設計 TTL/prune 機制
