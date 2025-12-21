#!/bin/zsh

setopt PROMPT_SUBST
ZLE_RPROMPT_INDENT=0

function set_full_prompt() {
    # 実行直後の終了ステータスを即座に保存
    local last_status=$?
    
    # 環境変数 LAST_STATUS として Rust 側に渡す
    # (Rust側の Command::new は親プロセスの環境変数を継承するため)
    PROMPT='$(LAST_STATUS='${last_status}' zsh-infinite zsh prompt left 2>/dev/null)'
    RPROMPT='$(LAST_STATUS='${last_status}' zsh-infinite zsh prompt right 2>/dev/null)'
}

function zle-line-finish() {
    local last_status=$?
    # Transient時も同様に環境変数を経由させるとRust側のロジックを統一できます
    PROMPT='$(LAST_STATUS='${last_status}' zsh-infinite zsh prompt transient 2>/dev/null)'
    RPROMPT=''
    zle reset-prompt
}
zle -N zle-line-finish

function precmd() {
    set_full_prompt
}

set_full_prompt
