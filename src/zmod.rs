use zsh_system::{Features, ZshModule, ZshResult, export_module};

#[derive(Default)]
struct ZshInfinite {}

impl ZshModule for ZshInfinite {
    fn setup(&mut self) -> ZshResult {
        // 初期化処理。エラー時は Err(Box::new(std::io::Error::...)) 等を返せる
        Ok(())
    }

    fn boot(&mut self) -> ZshResult {
        // 以前の 0 ではなく Ok(()) を返す
        Ok(())
    }

    fn features(&self) -> Features {
        // モジュールが提供する機能を定義
        Features::new()
    }

    fn cleanup(&mut self) -> ZshResult {
        // アンロード前のクリーンアップ
        Ok(())
    }

    fn finish(&mut self) -> ZshResult {
        // 最終的なメモリ解放など
        Ok(())
    }
}

// マクロを呼び出して C 言語用エントリポイントを生成
export_module!(ZshInfinite);
