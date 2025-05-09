// eva-tools library crate root. Provides reusable modules so that integration tests and other
// crates can depend on eva-tools logic.

pub mod combat;
pub mod tts_adapter;
pub mod fine_tune;

// You can publicly re-export functions for convenience.
pub use combat::simulate_battle;
pub use tts_adapter::speak;
