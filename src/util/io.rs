use std::path::Path;
use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};

pub fn read_utf16le(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path)?;
    let bytes = if bytes.starts_with(&[0xFF, 0xFE]) { &bytes[2..] } else { &bytes };
    let (text, _, had_errors) = encoding_rs::UTF_16LE.decode(bytes);
    if had_errors {
        anyhow::bail!("UTF-16LE decode error in {:?}", path);
    }
    Ok(text.into_owned())
}

pub fn write_utf16le(path: &Path, content: &str) -> Result<()> {
    let mut bytes = vec![0xFF, 0xFE]; // BOM
    for unit in content.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    std::fs::write(path, bytes)?;
    Ok(())
}

pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let text = read_utf16le(path)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn save_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let text = serde_json::to_string_pretty(value)?;
    write_utf16le(path, &text)
}
