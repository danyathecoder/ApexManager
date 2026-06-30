use std::path::{Path, PathBuf};

#[derive(Default, PartialEq, Clone)]
pub enum ServerStatus {
    #[default]
    Stopped,
    Running,
    Crashed(i32),
}

pub fn exe_path(server_dir: &Path) -> PathBuf {
    server_dir.join("accServer.exe")
}

pub fn start(server_dir: &Path) -> anyhow::Result<std::process::Child> {
    std::process::Command::new(exe_path(server_dir))
        .current_dir(server_dir)
        .spawn()
        .map_err(Into::into)
}

pub fn poll_status(child: &mut std::process::Child) -> ServerStatus {
    match child.try_wait() {
        Ok(Some(status)) => ServerStatus::Crashed(status.code().unwrap_or(-1)),
        Ok(None) => ServerStatus::Running,
        Err(_) => ServerStatus::Crashed(-1),
    }
}
