pub fn format_laptime(ms: u64) -> String {
    if ms == 0 || ms == u32::MAX as u64 {
        return "--:--.---".to_string();
    }
    let minutes = ms / 60_000;
    let seconds = (ms % 60_000) / 1_000;
    let millis = ms % 1_000;
    format!("{minutes}:{seconds:02}.{millis:03}")
}
