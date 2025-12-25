#!/bin/zsh

function get_git_status() {
    # Gitリポジトリ内にいるか確認
    if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then return; fi

    # 色の定義
    local reset_color="%{%f%}"
    local default_color="%{%F{white}%}"
    local clean_color="%{%F{green}%}"
    local staged_color="%{%F{yellow}%}"
    local unstaged_color="%{%F{red}%}"
    local untracked_color="%{%F{blue}%}"
    local conflict_color="%{%F{magenta}%}"
    local stash_color="%{%F{cyan}%}"
    local ahead_color="%{%F{cyan}%}"
    local behind_color="%{%F{magenta}%}"

    # リモートアイコンの設定
    local remote_icon="󰊢"
    local remote_url=$(git remote get-url origin 2>/dev/null)
    [[ "$remote_url" =~ "github.com" ]] && remote_icon=""
    [[ "$remote_url" =~ "gitlab.com" ]] && remote_icon=""

    # ブランチ名の取得
    local branch=$(git symbolic-ref --short HEAD 2>/dev/null || git rev-parse --short HEAD 2>/dev/null)

    # ステータスの取得 (一括取得して効率化)
    local status_output=$(git status --porcelain=v2 --branch 2>/dev/null)
    
    local ahead=0 behind=0 staged_changes=0 unstaged_changes=0 untracked_files=0 conflicts=0 stashed=0

    # 1. アップストリームの解析 (ahead/behind)
    # v2形式の # branch.ab +1 -0 を解析
    local ab_line=$(echo "$status_output" | grep '^# branch\.ab')
    if [[ "$ab_line" =~ " \+([0-9]+) -([0-9]+)" ]]; then
        ahead=${match[1]}
        behind=${match[2]}
    fi

    # 2. ファイル変更の解析
    # 改行で分割してループ
    while IFS= read -r line; do
        [[ -z "$line" ]] && continue
        local prefix=${line:0:1}
        
        case "$prefix" in
            "1"|"2") # 1:通常, 2:リネーム・コピー
                [[ ${line:2:1} != "." ]] && ((staged_changes++))
                [[ ${line:3:1} != "." ]] && ((unstaged_changes++))
                ;;
            "u") # 衝突 (Unmerged)
                ((conflicts++))
                ;;
            "?") # 追跡対象外
                ((untracked_files++))
                ;;
        esac
    done <<< "$(echo "$status_output" | grep -E '^[12u\?]')"

    # 3. スタッシュの確認
    git rev-parse --verify refs/stash >/dev/null 2>&1 && stashed=1

    # 色の優先順位判定
    local summary_color=$clean_color
    if (( conflicts > 0 )); then summary_color=$conflict_color
    elif (( unstaged_changes > 0 )); then summary_color=$unstaged_color
    elif (( untracked_files > 0 )); then summary_color=$untracked_color
    elif (( staged_changes > 0 )); then summary_color=$staged_color
    elif (( stashed > 0 )); then summary_color=$stash_color
    fi

    # 出力文字列の構築
    local status_icons=""
    (( staged_changes > 0 )) && status_icons+="+${staged_changes} "
    (( unstaged_changes > 0 )) && status_icons+="!${unstaged_changes} "
    (( untracked_files > 0 )) && status_icons+="?${untracked_files} "
    (( conflicts > 0 ))      && status_icons+="${conflicts} "
    (( stashed > 0 ))        && status_icons+=" "

    # クリーン判定
    if [[ -z "$status_icons" ]]; then
        status_icons="${clean_color}${reset_color}"
    else
        status_icons="${summary_color}${status_icons}${reset_color}"
    fi

    # プッシュ/プル状況
    local ab_icons=""
    (( ahead > 0 )) && ab_icons+=" ${ahead_color}↑${ahead}${reset_color}"
    (( behind > 0 )) && ab_icons+=" ${behind_color}↓${behind}${reset_color}"

    echo "${default_color}${remote_icon}  ${branch}${reset_color} ${status_icons}${ab_icons}"
}

get_git_status
