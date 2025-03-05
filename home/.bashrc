#
# ~/.bashrc
#

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

alias ls='ls --color=auto'
alias grep='grep --color=auto'
PS1='[\u@\h \W]\$ '

DOTFILES_HOME="$HOME/.dotfiles/home"
DOTFILES_ROOT="$HOME/.dotfiles/root"
alias stow_home="stow --dir='$DOTFILES_HOME' --target='$HOME/sample2'"
alias stow_root="stow --dir='$DOTFILES_ROOT' --target='/'"

alias fetch="neowofetch"
fetch
