// Integration test for tts_adapter.rs
// Please adjust the API usage according to your actual tts_adapter.rs implementation.
use eva_tools::tts_adapter;

#[test]
fn test_tts_basic() {
    // Example: test a basic TTS scenario
    // You may need to adapt this based on tts_adapter.rs API
    let result = tts_adapter::speak("Hello, world!");
    assert!(result.is_ok());
}
