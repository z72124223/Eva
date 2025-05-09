//! Skeleton for PlanExecutor

pub struct PlanExecutor;

impl PlanExecutor {
    pub fn new() -> Self {
        PlanExecutor
    }
    pub fn execute(&self) {
        // skeleton
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = PlanExecutor::new();
    }
}
