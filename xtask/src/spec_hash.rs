use std::fs;
use sha2::{Sha256, Digest};

pub fn gen_spec_hash() -> Result<String, String> {
    let spec = fs::read_to_string("ARCHITECTURE/ARCHITECTURE_SPEC.md").map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    hasher.update(spec.as_bytes());
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn write_spec_hash() -> Result<(), String> {
    let hash = gen_spec_hash()?;
    fs::write(".ci/spec_hash", &hash).map_err(|e| e.to_string())?;
    println!("[xtask] 已寫入 spec_hash: {}", hash);
    Ok(())
}
