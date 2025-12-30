use super::paths::get_daemon_paths;
use std::fs;

pub async fn stop() {
    let paths = get_daemon_paths();

    if let Ok(pid_str) = fs::read_to_string(&paths.pid) {
        if let Ok(pid) = pid_str.trim().parse::<i32>() {
            println!("Stopping daemon with PID: {}", pid);

            let result = unsafe { libc::kill(pid, 15) };

            if result == 0 {
                println!("Daemon stopped successfully.");
            } else {
                eprintln!("Failed to stop daemon. It might have already exited.");
            }

            // PIDファイルとソケットを掃除
            let _ = fs::remove_file(&paths.pid);
            let _ = fs::remove_file(&paths.socket);
            return;
        }
    }
    println!("No running daemon found (PID file missing).");
}
