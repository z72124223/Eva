# EVA External API Specification (OpenAPI 3.1)

本文件定義 EVA Gateway 對外 API 規格，支援 Plug-&-Play 任意前端對接。

---

## OpenAPI 3.1 YAML

```yaml
openapi: 3.1.0
info:
  title: EVA Gateway API
  version: 1.0.0
  description: |
    EVA Plug-&-Play Gateway，支援自動增生、AI 補檔、品質自評、即時監控。
servers:
  - url: http://localhost:8080
paths:
  /v1/infer:
    post:
      summary: LLM 內容生成
      description: 轉發 prompt 至 LLM，回傳生成內容
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              properties:
                prompt:
                  type: string
      responses:
        '200':
          description: LLM 生成結果
          content:
            application/json:
              schema:
                type: object
                properties:
                  result:
                    type: string
  /v1/upgrade:
    post:
      summary: 自動補檔/AI 增生
      description: 自動補齊缺檔、AI 產生骨架與測試
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              properties:
                action:
                  type: string
                mode:
                  type: string
      responses:
        '200':
          description: 補檔結果
          content:
            application/json:
              schema:
                type: object
                properties:
                  status:
                    type: string
                  detail:
                    type: string
  /v1/scene_score:
    post:
      summary: 劇情品質自評
      description: 對劇情片段進行自動評分
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              properties:
                scene:
                  type: string
      responses:
        '200':
          description: 評分結果
          content:
            application/json:
              schema:
                type: object
                properties:
                  score:
                    type: number
                  reason:
                    type: string
```

---

## Swagger/OpenAPI JSON

- 建議用 [utoipa](https://github.com/juhaku/utoipa) 或 [paperclip](https://github.com/wafflespeanut/paperclip) 自動從 Rust 註解產生 Swagger JSON，供前端/AI runtime 讀取。
- Swagger JSON 路徑建議： `/swagger.json` 或 `/openapi.json`

---

## 安全與認證
- 所有 API 需帶 JWT（Authorization: Bearer）
- QPS 流控、metrics 監控

---

## 版本與維護
- 本文件自動由 Gateway 產生，確保與 API 實作一致。
