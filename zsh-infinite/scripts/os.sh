#!/bin/zsh

function get_os_icon() {
    case "$(uname)" in
        Darwin) echo "" ;; # Apple icon
        Linux)
            if [ -f /etc/os-release ]; then
                if grep -q "ID=ubuntu" /etc/os-release; then echo "" # Ubuntu
                elif grep -q "ID=fedora" /etc/os-release; then echo "" # Fedora
                elif grep -q "ID=centos" /etc/os-release || grep -q "ID=rhel" /etc/os-release; then echo "" # CentOS/RHEL
                elif grep -qEi "(microsoft|wsl)" /proc/version >/dev/null 2>&1; then echo "" # WSL (Linux kernel, but Windows Subsystem)
                elif [ -f /etc/arch-release ]; then echo "󰣇" # Arch
                elif [ -f /etc/debian_version ]; then echo "" # Debian
                else echo "" # Generic Linux
                fi
            elif grep -qEi "(microsoft|wsl)" /proc/version >/dev/null 2>&1; then echo "" # WSL (fallback)
            elif [ -f /etc/arch-release ]; then echo "󰣇" # Arch
            elif [ -f /etc/debian_version ]; then echo "" # Debian
            else echo "" # Generic Linux
            fi
            ;;
        CYGWIN*|MINGW*) echo "" ;; # Windows (Cygwin/MSYS)
        *) echo "󰀵" ;; # Default
    esac
}
get_os_icon

