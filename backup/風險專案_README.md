# EVA 《互動小說》風險管理專案

版本：2025-05-04

---

## 1. 目標
1. **完整列舉所有產品風險**（含技術 / 故事 / 法規 / 營運）。
2. **資料化 (Config-as-Data) 管理**：風險條目存於 `risks/`，非硬編在程式。
3. **量化 (FMEA) + 模組對應**，支援自動排序 / 告警 / 迭代。
4. **可演進記憶體層**（插拔式 `MemoryStore`）。

---

## 2. 專案結構（建議）
```
│ 風險專案_README.md      # 本檔
│ eva.toml                # 新增 memory.strategy 等設定（主 repo）
├─risks/
│   categories.yaml       # 風險大類/細類定義（Data）
│   fmea.csv              # FMEA 表 (impact, likelihood, detectability, rpn…)
│   schema.json           # YAML/CSV 驗證 schema
├─src/memory/
│   mod.rs                # pub trait MemoryStore {...}
│   naive_log.rs          # 實作 1
│   summary_v1.rs         # 實作 2
│   retrieval.rs          # 實作 3
│   hybrid.rs             # 實作 4
└─.github/workflows/
    risk_ci.yml           # CI：schema 驗證 + RPN > 200 標籤
```

---

## 3. 新增風險（補充）
以下風險先前未詳述，建議納入 `categories.yaml`。

| 大類 | 細分類 | 典型風險 / 說明 |
|------|--------|------------------|
| **法規/版權** | 版權侵權 | AI 生成包含受保護文本 / 圖像 | 
|               | 法規限制 | 內容違反地方法律（賭博、仇恨…） | 
| **資料隱私**  | 個資洩漏 | 玩家輸入真實姓名被回放 | 
| **模型服務**  | API 速率限制 | OpenAI Rate Limit 導致中斷 | 
|               | 成本超支 | Token 用量失控 | 
| **維運穩定**  | 服務宕機 | LLM API 或自架模型離線 | 
|               | 依賴破壞 | 升級 reqwest 導致 TLS 失敗 | 
| **文化偏誤**  | 地域/族群偏見 | AI 輸出歧視性敘事 | 
| **安全攻擊**  | Prompt Injection | 玩家輸入惡意指令使 AI 洩漏系統提示 | 
| **版本兼容**  | 模型升級不兼容 | gpt-4o 行為改變破壞劇情邏輯 | 

*這些風險請在 `fmea.csv` 填入初始分數並持續調整。*

---

## 4. MemoryStore 插拔方案
見 `src/memory/mod.rs`：
```rust
pub trait MemoryStore {
    fn push_exchange(&mut self, role: &str, content: &str);
    fn build_context(&self, recent_n: usize, tokens_limit: usize) -> Vec<(String,String)>;
}
```
* StorySession 僅依賴 trait。
* 新策略＝新增檔案實作，不改其他程式。
* `eva.toml`
  ```toml
  [memory]
  strategy = "summary_v1"  # naive_log | summary_v1 | retrieval | hybrid
  recent_n = 10
  token_limit = 3000
  ```

---

## 5. CI / GitHub Action 建議
```yaml
name: Risk CI
on: [push, pull_request]
jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Check schema
        run: python scripts/validate_risks.py  # 檢查 YAML/CSV
      - name: High RPN warning
        run: python scripts/check_rpn.py      # RPN≥200 標籤
```

---

## 6. 下一步實踐路線
1. **建立目錄** `risks/`，填入 `categories.yaml` 與 `fmea.csv` (可從本文表格匯出)。
2. **撰寫 schema** (`schema.json`) + Python 驗證腳本。  
3. **實作 MemoryStore**：先 `naive_log` + `summary_v1`。  
4. **在 StorySession** 改用 `Box<dyn MemoryStore>`.  
5. **設定** `eva.toml` 預設策略。  
6. **CI** 上線風險驗證工作流。  
7. 第一輪測試 → 更新 FMEA 分值 → 規劃風險緩解模組。

---

> 後續如有新風險，僅需：
> 1. 編輯 `categories.yaml` / `fmea.csv`。
> 2. (必要) 實作對應模組並在對應表加註。
> 3. CI 自動驗證並提示高 RPN。

此 README 為風險專案指南，可隨著實作進度持續增修。
