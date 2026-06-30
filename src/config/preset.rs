use serde::{Deserialize, Serialize};

use crate::config::{
    assist_rules::AssistRules, bop::Bop, entry_list::EntryList, event::Event,
    event_rules::EventRules, server_config::ServerConfig, settings::Settings,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Preset {
    pub server_config: ServerConfig,
    pub settings: Settings,
    pub event: Event,
    pub event_rules: EventRules,
    pub assist_rules: AssistRules,
    pub entry_list: EntryList,
    pub bop: Bop,
}

pub fn presets_dir() -> Option<std::path::PathBuf> {
    dirs::config_dir().map(|d| d.join("ApexManager").join("presets"))
}

pub fn list_presets() -> Vec<String> {
    let Some(dir) = presets_dir() else { return Vec::new() };
    let Ok(entries) = std::fs::read_dir(&dir) else { return Vec::new() };
    let mut names: Vec<String> = entries
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) == Some("json") {
                p.file_stem().and_then(|s| s.to_str()).map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect();
    names.sort();
    names
}

pub fn save_preset(name: &str, preset: &Preset) -> anyhow::Result<()> {
    let dir = presets_dir().ok_or_else(|| anyhow::anyhow!("No config dir"))?;
    std::fs::create_dir_all(&dir)?;
    let text = serde_json::to_string_pretty(preset)?;
    std::fs::write(dir.join(format!("{name}.json")), text)?;
    Ok(())
}

pub fn load_preset(name: &str) -> anyhow::Result<Preset> {
    let dir = presets_dir().ok_or_else(|| anyhow::anyhow!("No config dir"))?;
    let text = std::fs::read_to_string(dir.join(format!("{name}.json")))?;
    Ok(serde_json::from_str(&text)?)
}

pub fn delete_preset(name: &str) -> anyhow::Result<()> {
    let dir = presets_dir().ok_or_else(|| anyhow::anyhow!("No config dir"))?;
    std::fs::remove_file(dir.join(format!("{name}.json")))?;
    Ok(())
}
