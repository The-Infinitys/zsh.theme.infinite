# --- dev.zshrc ---

# 共通設定: バイナリへのパスを通す
export PATH="{{RUN_DIR}}:$PATH"
alias zsh-infinite="{{RUN_DIR}}/zsh-infinite"

# モード選択の入力を促す
echo "------------------------------------------"
echo "  zsh-infinite: Select execution mode"
echo "  [1] Library (Zmodload - Faster)"
echo "  [2] Binary  (Command line - Standard)"
echo "------------------------------------------"
echo -n "Select (1/2) [default: 1]: "
read -k 1 res
echo ""

if [[ "$res" == "2" ]]; then
    # --- バイナリモード ---
    echo ">>> Mode: Binary"
    source "{{RUN_DIR}}/.zsh-theme"
else
    # --- ライブラリモード ---
    echo ">>> Mode: Library"
    
    # 1. module_path の設定
    # ライブラリがビルドされている target/debug を検索パスに追加
    # ({{RUN_DIR}} の一つ上の target/debug を指すように調整)
    module_path=("{{RUN_DIR}}/../target/debug" $module_path)

    # 2. zsh_infinite モジュールのロード
    # module_path が設定されているので、ファイル名ではなくモジュール名でロード可能
    if zmodload libzsh_infinite; then
        echo "Successfully loaded zsh_infinite module."
        # モジュール版は Rust 側で PROMPT 変数を直接管理するため、
        # setopt PROMPT_SUBST のみ有効化する
        setopt PROMPT_SUBST
    else
        echo "Error: Failed to load zsh_infinite module from module_path."
        # echo "Falling back to Binary mode..."
        # source "{{RUN_DIR}}/.zsh-theme"
    fi
fi

echo "------------------------------------------"
