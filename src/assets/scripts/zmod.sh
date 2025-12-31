#!/bin/zsh

# 2. ラッパー関数の定義
# add-zsh-hook は関数を要求するため、ビルトインを直接登録せず関数を経由させる
_zsh_infinite_precmd_wrapper() {
    if (( $+builtins[rust_status] )); then
        rust_status "$@"
    fi
}

# 3. add-zsh-hook のロードと登録
if autoload -Uz add-zsh-hook 2>/dev/null; then
    # 重複登録を防ぎつつ登録
    add-zsh-hook precmd _zsh_infinite_precmd_wrapper
else
    # add-zsh-hook 自体が見つからない場合のフォールバック (配列へ直接注入)
    if [[ "${precmd_functions[(r)_zsh_infinite_precmd_wrapper]}" != "_zsh_infinite_precmd_wrapper" ]]; then
        typeset -ga precmd_functions
        precmd_functions=(_zsh_infinite_precmd_wrapper $precmd_functions)
    fi
fi

# デバッグ用メッセージ
echo "[ZshInfinite] Hook initialization complete."