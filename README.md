# EVA Workspace

## EVA 嚴格驗證與自動化藍圖原則

- EVA 強調全流程自動化、嚴格一致性與架構整潔。
- **所有底層（如 LLM/GPU）異常，必須 fail fast，立即阻斷流程並明確提示修復步驟。**
- 嚴禁 silent fallback（如自動降級為 CPU），所有 workaround 必須有明確審核與記錄。
- 每一層（如 GPU 啟用、LLM 串接）都必須自動驗證、可追溯、可複現。
- EVA 啟動時會自動檢查 GPU、CUDA、模型格式、API 串接等，缺失時阻斷並回報。
- 任何 workaround、降級、繞過都需有明確審核與記錄，不能默默發生。
- 目標：讓每一位 AI 開發者一進來就能明確理解這些原則，確保專案品質與可維護性。


此目錄已初始化為 Rust workspace 結構，建議後續於 `Cargo.toml` 補上 workspace 配置，並將各 crate 放於 `crates/` 目錄下。

## 建議目錄結構

```
Eva/
├── ARCHITECTURE_TREE_CANONICAL.md   # 最新 workspace tree 結構，請參考此檔
├── README.md
├── Cargo.toml           # [workspace] 配置
├── eva/                 # 主程式 crate
├── crates/              # 子模組 crate
│   ├── domain/
│   ├── services/
│   ├── infrastructure/
│   └── utils/
└── ...
```

## 初始化 workspace

1. 建立 `crates/` 目錄：
   ```sh
   mkdir crates
   ```
2. 編輯根目錄 `Cargo.toml`：
   ```toml
   [workspace]
   members = [
     "eva",
     "crates/*"
   ]
   ```
3. 每個 crate 皆可獨立 `cargo test`、`cargo build`。

---

> **注意：**
> - [總覽與目錄結構](./ARCHITECTURE_TREE_CANONICAL.md)，該檔已精簡僅作為導向。
> - 最新 tree 結構、目錄索引，請一律參考 ARCHITECTURE_TREE_CANONICAL.md。

如需自動生成模組骨架、CI/CD、或 Plug & Play Gateway 範例，請隨時告知！
