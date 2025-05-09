# EVA Architecture Specification (Spec)

本文件為唯一規範來源，所有規則、流程、驗收條件、權限模型、CI 標準等皆以此為準。

## 強制規範
- 見 ARCHITECTURE_RULES.md 條文（自動同步）

## 流程條文
- PR、CI、修補、審查、patch-safety、記憶 API 權限、Ops 監控等，詳見各專章

### WebSocket Listener → Intent channel 流程圖

```mermaid
flowchart LR
    subgraph Client
        A[WebSocket Client]
    end
    subgraph Core
        B[WebSocket Listener]
        C[Intent channel (mpsc)]
        D[Intent Consumer]
    end
    A -- ws://0.0.0.0:9898/ws --> B
    B -- send intent --> C
    C -- receive intent --> D
```

- WebSocket Listener 接收來自 Client 的訊息
- 經 intent channel 傳遞給核心消費端

## 驗收標準
- 里程碑之 Definition of Done
- 覆蓋率、mutation testing、layout check、spec hash check

## 權限與隱私
- owner_id/scope 欄位、GDPR/CCPA 流程、API 權限

## 其他
- 任何改動本文件必須先走 RFC/PR 流程，並通過 CI hash check
