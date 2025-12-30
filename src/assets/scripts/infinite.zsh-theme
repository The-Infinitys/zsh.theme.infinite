#!/bin/zsh

# プロンプトの置換を有効化
setopt PROMPT_SUBST
# 右プロンプトの右端の空白を詰める
ZLE_RPROMPT_INDENT=0

# プロンプト更新用のメイン関数
function _update_infinite_prompt() {
    local last_status=$?
    
    # --- カーソルリセットを実行 ---
    _reset_cursor
    
    PROMPT='$(LAST_COMMAND_EXECUTED='$LAST_COMMAND_EXECUTED' LAST_STATUS='${last_status}' zsh-infinite zsh prompt left 2>/dev/null)'
    RPROMPT='$(LAST_COMMAND_EXECUTED='$LAST_COMMAND_EXECUTED' LAST_STATUS='${last_status}' zsh-infinite zsh prompt right 2>/dev/null)'
    print -P $PROMPT

}

# コマンド確定時（エンターキー押下時）の処理
function _infinite_transient_prompt() {
    local last_status=$?
    export LAST_COMMAND_EXECUTED=$EPOCHREALTIME
    zsh-infinite zsh prompt hook 2> /dev/null
    PROMPT='$(LAST_STATUS='${last_status}' zsh-infinite zsh prompt transient 2>/dev/null)'
    RPROMPT=''
    zle reset-prompt
}

# カーソル形状をデフォルト（ブロック等）に戻す
function _reset_cursor() {
    # \e[0 q はターミナル設定のデフォルトに戻すエスケープシーケンス
    echo -ne '\e[0 q'
}

# 既存のフックを上書きしないように配列に追加
autoload -Uz add-zsh-hook
# precmdはコマンド実行後、次のプロンプトが表示される直前に呼ばれます
add-zsh-hook precmd _update_infinite_prompt

# Transient Prompt 用のウィジェット登録
zle -N zle-line-finish _infinite_transient_prompt
