use serde::{Deserialize};
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct SafetyConfig {
    pub MAX_PATCH_SIZE: usize,
    pub danger_list: Vec<String>,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        SafetyConfig {
            MAX_PATCH_SIZE: 2048,
            danger_list: vec![
                "unsafe".to_string(),
                "std::fs::remove_".to_string(),
                "std::process::Command".to_string(),
            ],
        }
    }
}

impl SafetyConfig {
    pub fn from_yaml(path: &str) -> Self {
        match fs::read_to_string(path) {
            Ok(content) => serde_yaml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_load() {
        // crate 路徑: crates/eva-tools -> 專案根 safety.yaml 位於 ../../safety.yaml
        let config = SafetyConfig::from_yaml("../../safety.yaml");
        assert_eq!(config.MAX_PATCH_SIZE, 2048);
        assert!(config.danger_list.contains(&"unsafe".to_string()));
    }
}
