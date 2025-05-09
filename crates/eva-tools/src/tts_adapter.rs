//! Simple stub TTS adapter used for integration testing.
//! Replace with real TTS implementation when available.

use anyhow::{Result};

/// Speak text using TTS engine. For tests we just return Ok(())
pub fn speak(text: &str) -> Result<()> {
    if text.trim().is_empty() {
        anyhow::bail!("text is empty");
    }
    // In real implementation call TTS engine.
    Ok(())
}
