#
# ~/.bashrc
#

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

# Load profiles from ~/profile.d
if test -d "$HOME/.profile.d"; then
    for profile in "$HOME/.profile.d/{local,portable}/*.sh"; do
        test -r "$profile" && . "$profile"
    done
    unset profile
fi

# Auto run
fetch
