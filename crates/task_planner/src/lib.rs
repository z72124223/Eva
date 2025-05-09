use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub desc: String,
    pub depends_on: Vec<usize>,
}

/// 最小可用 rule-based 任務拆解
pub fn plan(goal: &str) -> Vec<Task> {
    // 沙盒：將 goal 拆成三個子任務
    vec![
        Task {
            id: 1,
            title: format!("分析目標: {}", goal),
            desc: "理解並拆解需求".to_string(),
            depends_on: vec![],
        },
        Task {
            id: 2,
            title: "設計方案".to_string(),
            desc: "根據需求設計解決步驟".to_string(),
            depends_on: vec![1],
        },
        Task {
            id: 3,
            title: "實作與驗證".to_string(),
            desc: "根據方案進行實作與測試".to_string(),
            depends_on: vec![2],
        },
    ]
}
