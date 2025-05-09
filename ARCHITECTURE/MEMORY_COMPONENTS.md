# EVA 記憶系統組件現況

| 組件             | 目前狀態                                   | 缺口                                              |
|------------------|--------------------------------------------|---------------------------------------------------|
| 短期記憶 (ContextMemory) | WP-6 尚未實作，只有 log                    | 需在 Executor middleware 寫入                    |
| 長期記憶 (VectorMemory)  | 無                                         | 需 Embedding + 向量 DB (Qdrant/Milvus/SQLite-pgv) |
| 記憶整理 (Consolidator)   | 無                                         | 夜間 Job：將 N 條短期記憶 → 摘要 + 寫入長期        |
| 遺忘 / TTL               | 無                                         | 定期壓縮或 TTL 刪除，防 index 膨脹                |
| 隱私 & 權限              | RFC 已列條款，但尚未碼進程式               | 需中介層檢查 owner_id / scope                     |

## 工作任務
- [ ] ContextMemory: Executor middleware 完成記憶寫入
- [ ] VectorMemory: 建立 embedding 與向量 DB 整合
- [ ] Consolidator: 開發夜間記憶整理與摘要 job
- [ ] TTL/Prune: 設計定期壓縮、TTL 刪除機制
- [ ] 隱私權限: 實作 owner_id/scope 檢查邏輯
