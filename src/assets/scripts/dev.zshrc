#!/bin/zsh

export ZSH="${ZSH:-{{RUN_DIR}}/.oh-my-zsh}"
export ZSH_THEME="infinite"

ZSH_CUSTOM="{{RUN_DIR}}"

source $ZSH/oh-my-zsh.sh

fpath=({{RUN_DIR}} $fpath)

# Theme
source ${ZSH_CUSTOM}/infinite.zsh-theme
