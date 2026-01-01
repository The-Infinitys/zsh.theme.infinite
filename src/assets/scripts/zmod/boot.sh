#!/bin/zsh
unsetopt PROMPT_SUBST
ZLE_RPROMPT_INDENT=0


# --- プロンプト更新フック ---
function _zsh_infinite_precmd() {
    __zsh_infinite_internal precmd >/dev/null 2>&1

}

# --- Transient Prompt ---
function _zle_infinite_line_finish() {
    __zsh_infinite_internal line-finish 2>/dev/null 
}

{
    autoload -Uz add-zsh-hook
    add-zsh-hook precmd _zsh_infinite_precmd
    local current_widget="${widgets[zle-line-finish]}"
    local old_func=""

    if [[ "$current_widget" == "user:"* ]]; then
        # "user:_original_func" から関数名部分だけを抽出
        old_func="${current_widget#user:}"
    fi
    __zsh_infinite_internal store zle-line-finish "${old_func:-${current_widget}}"
    
    zle -N zle-line-finish _zle_infinite_line_finish
} >/dev/null 2>&1
