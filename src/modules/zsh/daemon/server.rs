use tokio::{
    io::AsyncReadExt,
    net::{UnixListener, UnixStream},
};

pub async fn server(socket_path: std::path::PathBuf) {
    let _ = tokio::fs::remove_file(&socket_path).await;
    let listener = UnixListener::bind(&socket_path).expect("Failed to bind socket");
    println!("DEBUG: Server listening on {:?}", socket_path);

    loop {
        println!("DEBUG: Waiting for accept...");
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("DEBUG: Accepted connection!");
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream).await {
                        println!("DEBUG: handle_client error: {}", e);
                    }
                });
            }
            Err(e) => println!("DEBUG: Accept error: {}", e),
        }
    }
}

async fn handle_client(mut stream: UnixStream) -> Result<(), Box<dyn std::error::Error>> {
    use rkyv::from_bytes;
    use rkyv::rancor::Error;
    use rkyv::to_bytes;
    use tokio::io::AsyncWriteExt;
    use zsh_prompts::Commands;
    let mut len_buf = [0u8; 8];
    stream.read_exact(&mut len_buf).await?;
    let len = u64::from_le_bytes(len_buf) as usize;
    println!("Server: Expecting {} bytes", len); // ログ

    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;
    println!("Server: Data received. Deserializing...");

    let command = from_bytes::<Commands, Error>(&buf)?;
    println!("Server: Command executed...");
    let segments = command.exec();

    let bytes = to_bytes::<Error>(&segments)?;
    println!("Server: Sending response ({} bytes)", bytes.len());

    stream
        .write_all(&(bytes.len() as u64).to_le_bytes())
        .await?;
    stream.write_all(&bytes).await?;
    Ok(())
}
