mod safety_config;

fn main() {
    let config = safety_config::SafetyConfig::from_yaml("../../../safety.yaml");
    println!("MAX_PATCH_SIZE: {}", config.MAX_PATCH_SIZE);
    println!("danger_list: {:?}", config.danger_list);
}
