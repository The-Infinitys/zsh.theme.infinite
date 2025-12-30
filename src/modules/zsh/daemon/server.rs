use tokio::{
    io::AsyncReadExt,
    net::{UnixListener, UnixStream},
};

pub async fn server(socket_path: std::path::PathBuf) {
    // 古いソケットファイルを削除してBind
    let _ = tokio::fs::remove_file(&socket_path).await;
    let listener = UnixListener::bind(&socket_path).expect("Failed to bind socket");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Accept error: {}", e),
        }
    }
}

async fn handle_client(mut stream: UnixStream) -> Result<(), Box<dyn std::error::Error>> {
    // 1. リクエストのサイズ (u64 / 8bytes) を読み取る
    let mut len_buf = [0u8; 8];
    stream.read_exact(&mut len_buf).await?;
    let len = u64::from_le_bytes(len_buf) as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;
    Ok(())
}
