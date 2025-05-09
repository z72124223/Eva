/// Intent → Plan 純解析 trait，不得使用任何執行期狀態
pub trait IntentParser {
    type Plan;
    fn parse_intent(&self, input: &str) -> Self::Plan;
}

/// Plan → Act 功能調度 trait，允許依需求注入執行期狀態
pub trait PlanExecutor<P> {
    type Output;
    fn execute_plan(&self, plan: P) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, PartialEq)]
    enum DummyPlan { Sum, Greet(String) }
    struct DummyParser;
    impl IntentParser for DummyParser {
        type Plan = DummyPlan;
        fn parse_intent(&self, input: &str) -> Self::Plan {
            if input.contains("sum") {
                DummyPlan::Sum
            } else {
                DummyPlan::Greet(input.trim().to_string())
            }
        }
    }
    struct DummyExecutor;
    impl PlanExecutor<DummyPlan> for DummyExecutor {
        type Output = String;
        fn execute_plan(&self, plan: DummyPlan) -> Self::Output {
            match plan {
                DummyPlan::Sum => "result: 42".to_string(),
                DummyPlan::Greet(name) => format!("hi, {}!", name),
            }
        }
    }
    #[test]
    fn test_intent_plan_split() {
        let parser = DummyParser;
        let exec = DummyExecutor;
        let plan = parser.parse_intent("sum");
        assert_eq!(plan, DummyPlan::Sum);
        let res = exec.execute_plan(plan);
        assert_eq!(res, "result: 42");
        let plan2 = parser.parse_intent("Alice");
        let res2 = exec.execute_plan(plan2);
        assert_eq!(res2, "hi, Alice!");
    }
}
