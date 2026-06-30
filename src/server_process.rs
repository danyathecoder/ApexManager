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
        Ok(Some(status)) => {
            let code = status.code().unwrap_or(-1);
            // 0xC000013A = STATUS_CONTROL_C_EXIT: user closed the console window or sent Ctrl+C
            if code == 0 || code == -1073741510 {
                ServerStatus::Stopped
            } else {
                ServerStatus::Crashed(code)
            }
        }
        Ok(None) => ServerStatus::Running,
        Err(_) => ServerStatus::Crashed(-1),
    }
}
