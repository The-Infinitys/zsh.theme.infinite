#!/bin/zsh

function get_smart_pwd() {
    local dir_path="${PWD/#$HOME/~}"
    local icon=""
    
    # 書き込み権限がない場合に鍵マーク
    [[ ! -w . ]] && icon=""
    [[ "$dir_path" == "~" ]] && icon=""

    # 長さ制限（例: 20文字以上なら各要素を省略）
    local max_len=100
    if (( ${#dir_path} > max_len )); then
        # ディレクトリを / で分割し、各要素の中間を ... にする処理
        # (複雑化を避けるため、ここでは簡易的なパス短縮を例示)
        echo "$icon $(echo $dir_path | sed "s:\([^/][^/]\)[^/]*/:\1…/:g")"
    else
        echo "$icon $dir_path"
    fi
}

get_smart_pwd
