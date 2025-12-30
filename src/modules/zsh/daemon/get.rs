use super::paths::get_daemon_paths;
use rkyv::rancor::Error;
use rkyv::util::AlignedVec;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::time::{Duration, timeout};
use zsh_prompts::{Commands, PromptSegment}; // 前に作成したパス取得関数

pub async fn get(command: &Commands) -> Vec<PromptSegment> {
    let paths = get_daemon_paths();

    // 接続から受信完了までを 50ms〜100ms 程度で制限する
    let res = timeout(Duration::from_millis(500), async {
        let mut stream = UnixStream::connect(&paths.socket).await.ok()?;

        // リクエスト送信
        let request_bytes = rkyv::to_bytes::<Error>(command).ok()?;
        stream
            .write_all(&(request_bytes.len() as u64).to_le_bytes())
            .await
            .ok()?;
        stream.write_all(&request_bytes).await.ok()?;

        // レスポンスサイズ受信
        let mut res_len_buf = [0u8; 8];
        stream.read_exact(&mut res_len_buf).await.ok()?;
        let res_len = u64::from_le_bytes(res_len_buf) as usize;

        // 本体受信 (AlignedVecを使いつつ安全に)
        let mut res_buf = AlignedVec::<16>::with_capacity(res_len);
        // 安全にバッファを埋めるために一工夫
        let mut temp_buf = vec![0u8; res_len];
        stream.read_exact(&mut temp_buf).await.ok()?;
        res_buf.extend_from_slice(&temp_buf);

        rkyv::from_bytes::<Vec<PromptSegment>, Error>(&res_buf).ok()
    })
    .await;

    match res {
        Ok(Some(segments)) => segments,
        Ok(None) => Vec::new(),
        Err(e) => {
            eprintln!("Timeout: {}", e);
            Vec::new()
        }
    }
}
