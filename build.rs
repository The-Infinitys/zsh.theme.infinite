// プロジェクトルートの build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap(); // debug or release

    // ライブラリの出力先を決定
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let lib_name = "zsh_infinite";
    let (prefix, ext) = if target_os == "macos" {
        ("lib", "dylib")
    } else {
        ("lib", "so")
    };

    // 通常、同じワークスペースなら target/{profile}/ 配下に生成されます
    // ワークスペース構成に合わせてパスを調整してください
    let lib_path = PathBuf::from(&manifest_dir)
        .join("target")
        .join(&profile)
        .join(format!("{}{}.{}", prefix, lib_name, ext));

    // バイナリコンパイル時に参照できるように環境変数をセット
    println!("cargo:rustc-env=ZSH_LIB_PATH={}", lib_path.display());

    // ライブラリが変更されたら再ビルドするように指示
    println!("cargo:rerun-if-changed={}", lib_path.display());
}
