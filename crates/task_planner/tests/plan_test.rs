use task_planner::{plan, Task};

#[test]
fn test_plan_basic() {
    let goal = "建立一個 Rust 專案並實作加法功能";
    let tasks = plan(goal);
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].id, 1);
    assert!(tasks[0].title.contains(goal));
}
