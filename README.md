# EVA Workspace

此目錄已初始化為 Rust workspace 結構，建議後續於 `Cargo.toml` 補上 workspace 配置，並將各 crate 放於 `crates/` 目錄下。

## 建議目錄結構

```
Eva/
├── ARCHITECTURE.md
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

如需自動生成模組骨架、CI/CD、或 Plug & Play Gateway 範例，請隨時告知！
