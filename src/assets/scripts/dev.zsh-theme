#!/bin/zsh

# PROMPT_SUBST オプションを設定して、プロンプト内のコマンド置換を有効にする
setopt PROMPT_SUBST

PROMPT='$( {{RUN_DIR}}/zsh-infinite zsh prompt left 2>/dev/null)'

RPROMPT='$( {{RUN_DIR}}/zsh-infinite zsh prompt right 2>/dev/null)'