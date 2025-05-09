//! Simple stub combat module used for integration testing.
//! In future, replace with your actual combat simulation logic.

use anyhow::{Result, anyhow};

/// Simulate a battle between two combatants and return a short outcome string.
/// Currently returns a deterministic message so that tests can assert on `Ok`.
pub fn simulate_battle(hero: &str, enemy: &str) -> Result<String> {
    if hero.is_empty() || enemy.is_empty() {
        return Err(anyhow!("Both combatants must be non-empty"));
    }
    // Very naive outcome for demo purpose.
    Ok(format!("{hero} defeated {enemy}!"))
}
