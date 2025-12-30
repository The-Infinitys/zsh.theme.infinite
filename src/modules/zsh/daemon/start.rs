use super::paths::get_daemon_paths;
use super::server;
use daemonize::Daemonize;
use std::env;
pub async fn start() {
    let paths = get_daemon_paths();

    // 既にソケットやPIDがある場合のクリーンアップ（古いプロセスの死骸対策）
    if paths.pid.exists() {
        println!("Warning: PID file exists. Is the daemon already running?");
    }

    let daemonize = Daemonize::new()
        .pid_file(&paths.pid)
        .chown_pid_file(true)
        .working_directory(env::current_dir().unwrap())
        .umask(0o077); // 本人以外読み書き不可

    match daemonize.start() {
        Ok(_) => {
            server::server(paths.socket).await;
        }
        Err(e) => eprintln!("Error starting daemon: {}", e),
    }
}
