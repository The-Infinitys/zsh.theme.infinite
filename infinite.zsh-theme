# infinite.zsh-theme
#
# This is a basic template for a Zsh theme.
# Customize it to fit your preferences.

# ------------------------------------------------------------------------------
# 1. PROMPT CUSTOMIZATION
#    Define your left and right prompts here.
#    See: https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html
# ------------------------------------------------------------------------------

# Example Prompt: User@Host:CurrentDirectory GitBranch
# Requires git plugin for git_prompt_info function, or define your own.

# Define some colors for the prompt (optional)
# See: https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html#Colors-and-Styles
# %{%F{color}%} - foreground color
# %{%B%} - bold
# %{%b%} - bold off
# %{%f%} - foreground color off

# PROMPT_DIRTRIM=3 # Trim directory paths to 3 components

# Zsh Theme function (optional, but good practice for complex prompts)
function prompt_infinite_theme() {
  # Current time (HH:MM)
  local time="%( कोरिया時間: %*)% %"
  
  # User and Host
  local user_host="%{%F{green}%}%n%{%f%}:%{%F{blue}%}%m%{%f%}"
  
  # Current working directory
  local current_dir="%{%F{cyan}%}%~%{%f%}"

  # Git status (if using a git plugin that provides git_prompt_info)
  # Otherwise, you might need to implement your own git status check.
  local git_status=""
  if (( $+functions[git_prompt_info] )); then
    git_status="$(git_prompt_info)"
  elif (( $+functions[__git_ps1] )); then
    # For themes that use bash-like git prompt
    git_status=$(__git_ps1 " (%s)")
  fi
  local git_display="%{%F{yellow}%}${git_status}%{%f%}"

  # Define the left prompt
  PROMPT="${user_host} ${current_dir}${git_display}\n${time} %# "

  # Define the right prompt (optional)
  # RPROMPT=" %{%F{magenta}%}⚡%{%f%}"
}

# Call the function to set the prompt
prompt_infinite_theme

# Ensure prompt is updated
# PROMPT_COMMAND="prompt_infinite_theme" # For themes that need dynamic updates

# ------------------------------------------------------------------------------
# 2. ALIASES AND FUNCTIONS
#    Define any custom aliases or functions here.
# ------------------------------------------------------------------------------

# Example alias
# alias ll='ls -alF'

# Example function
# function myfunc() {
#   echo "Hello from myfunc!"
# }

# ------------------------------------------------------------------------------
# 3. ZSH OPTIONS
#    Set any specific Zsh options here.
#    See: https://zsh.sourceforge.io/Doc/Release/Options.html
# ------------------------------------------------------------------------------

# Example options
# setopt auto_cd           # Change directory without typing 'cd'
# setopt nomatch           # Prevent globbing from matching no files

# ------------------------------------------------------------------------------
# 4. KEY BINDINGS (Optional)
#    Define custom key bindings.
#    See: https://zsh.sourceforge.io/Doc/Release/Zsh-Line-Editor.html#Zle-Builtins
# ------------------------------------------------------------------------------

# Example key binding: Ctrl+K to clear screen
# bindkey '^K' clear-screen

# ------------------------------------------------------------------------------
# 5. PLUGIN CONFIGURATION (Optional)
#    If your theme depends on or integrates with specific plugins (e.g., zsh-autosuggestions, zsh-syntax-highlighting),
#    you might include some basic configuration here.
#    Note: Plugins are usually managed by a plugin manager (e.g., Oh My Zsh, Antigen, Zinit).
# ------------------------------------------------------------------------------

# Example for zsh-autosuggestions:
# ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE='fg=244' # Grey suggestions

# Example for zsh-syntax-highlighting:
# ZSH_HIGHLIGHT_STYLES[command]='fg=green'

# ------------------------------------------------------------------------------
# 6. Theme Activation Hook (Optional)
#    Some themes might want to do something when activated.
# ------------------------------------------------------------------------------
# if [[ -n "$ZSH_THEME_INFINITE_ACTIVATED" ]]; then
#   # Run some setup if this theme is activated
#   echo "Infinite theme activated!"
# fi
