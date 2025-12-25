#!/bin/zsh
# 浮動小数点演算と時間の計算に必要なモジュールをロード
zmodload zsh/datetime
zmodload zsh/mathfunc

function get_execution_info() {
    local last_status=$LAST_STATUS
    local info=""

    # 色の定義
    local reset_color="%{%f%}"
    local success_color="%{%F{green}%}"       # 通常成功 (緑)
    local long_success_color="%{%F{yellow}%}"  # 時間超過成功 (黄緑)
    local failure_color="%{%F{red}%}"        # 通常失敗 (オレンジ)
    local long_failure_color="%{%F{red}%}"    # 時間超過失敗 (赤)

    local threshold_time=1.0 # 時間超過とみなす秒数

    local current_color="$reset_color"
    local status_icon="" # 成功アイコン

    # 実行時間の計算
    local timer_start=$LAST_COMMAND_EXECUTED
    local delta=0.0
    local duration_str=""

    if [[ -n "$timer_start" ]]; then
        local timer_now=$EPOCHREALTIME
        # 浮動小数点として計算
        delta=$(( timer_now - timer_start ))

        if (( delta >= 0.1 )); then # 0.1秒以上の場合のみ時間を表示
            local -i d=$(( int(delta / 86400) ))
            local -i h=$(( int(delta / 3600) % 24 ))
            local -i m=$(( int(delta / 60) % 60 ))
            local -i s=$(( int(delta) % 60 ))
            local ms=$(( int((delta - int(delta)) * 1000) )) # ミリ秒

            # 大きな単位から順に結合
            [[ $d -gt 0 ]] && duration_str+="${d}d"
            [[ $h -gt 0 ]] && duration_str+="${h}h"
            [[ $m -gt 0 ]] && duration_str+="${m}m"
            if [[ -z "$duration_str" ]]; then # 日、時、分がなければ秒とミリ秒を表示
                duration_str+="${s}.${(l:3:0:)ms}s"
            else # 日、時、分があれば秒まで
                duration_str+="${s}s"
            fi
        fi
    fi

    # 1. ステータス表示の判定と色付け
    if [[ $last_status -eq 0 ]]; then
        # 成功
        status_icon=""
        if (( delta >= threshold_time )); then
            current_color="$long_success_color" # 時間超過成功 (黄緑)
        else
            current_color="$success_color"      # 通常成功 (緑)
        fi
    else
        # 失敗
        status_icon=""
        if (( delta >= threshold_time )); then
            current_color="$long_failure_color" # 時間超過失敗 (赤)
        else
            current_color="$failure_color"      # 通常失敗 (オレンジ)
        fi
        info="$status_icon $last_status" # 失敗時はステータスコードも表示
    fi

    # 最終的な出力文字列の構築
    if [[ -n "$duration_str" ]]; then
        info="$current_color$status_icon $duration_str$reset_color"
    else
        info="$current_color$status_icon$reset_color"
    fi
    
    echo "$info"
}

get_execution_info


