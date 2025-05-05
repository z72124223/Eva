# external_api.md

## AI 管理員 API

### /api/ai_manager/infer
- 說明：讓指定 AI persona 進行推理
- 範例：
```json
POST /api/ai_manager/infer
{
  "persona": "EvaSys",
  "mode": "system_directive",
  "input": "請切換至節能模式"
}
```

### /api/ai_manager/update_memory
- 說明：更新 AI 記憶
- 範例：
```json
POST /api/ai_manager/update_memory
{
  "persona_id": "npc_01",
  "memory": { "player_like": "推理劇情", "last_branch": "ep02_battle" }
}
```

## 遊戲助手 API

### /api/game_assistant/ask
- 說明：玩家詢問遊戲助手
- 範例：
```json
POST /api/game_assistant/ask
{
  "player_id": "...",
  "question": "目前有哪些分支？"
}
```
