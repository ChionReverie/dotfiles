DOTFILES_HOME=$(dirname $(readlink -e ${BASH_SOURCE}))
cd $DOTFILES_HOME/auto/
DOTFILES_HOME=$DOTFILES_HOME cargo run
