use std::env;
use std::fs;
use std::path::PathBuf;

pub struct DaemonPaths {
    pub socket: PathBuf,
    pub pid: PathBuf,
}

pub fn get_daemon_paths() -> DaemonPaths {
    let uid = unsafe { libc::getuid() };
    let user = env::var("USER").unwrap_or_else(|_| "unknown".into());

    // 1. XDG_RUNTIME_DIR or /run/user/{uid}
    let base_dir = env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let run_user = PathBuf::from(format!("/run/user/{}", uid));
            if run_user.exists() {
                run_user
            } else {
                // 2. Fallback to /tmp/{user}
                let tmp_dir = PathBuf::from(format!("/tmp/zsh-infinite-{}", user));
                let _ = fs::create_dir_all(&tmp_dir); // フォルダ作成
                // パーミッションを 700 に (本人以外アクセス不可)
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(&tmp_dir, fs::Permissions::from_mode(0o700));
                tmp_dir
            }
        });

    DaemonPaths {
        socket: base_dir.join("zsh-infinite.sock"),
        pid: base_dir.join("zsh-infinite.pid"),
    }
}
