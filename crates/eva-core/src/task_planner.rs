//! Skeleton for TaskPlanner

#[derive(Debug, PartialEq)]
pub enum Plan {
    RandomStart,
}

pub struct TaskPlanner;

impl TaskPlanner {
    pub fn new() -> Self {
        TaskPlanner
    }
    /// 固定回傳 Plan::RandomStart
    pub fn plan(&self, _intent: &str) -> Plan {
        Plan::RandomStart
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plan_always_random_start() {
        let planner = TaskPlanner::new();
        assert_eq!(planner.plan("anything"), Plan::RandomStart);
        assert_eq!(planner.plan("random start"), Plan::RandomStart);
    }
    #[test]
    fn test_new() {
        let _ = TaskPlanner::new();
    }
}
