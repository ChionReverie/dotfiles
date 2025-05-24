# Chion's Dotfiles

## Getting Started
Start with the documentation [index](docs/00-index.md) for more details.

## Project Structure
* `docs` - Further documentation and instructions on how I set things up. 
* `home` - Settings and assets placed relative to the user's `$HOME` directory (Example: `/home/chion`)
* `root` - Settings and assets which should be placed relative to the user's root directory (IE `/`)
* `script` - Shell scripts which may be used in automating installation.

## Adoping configs
Updating the configs relies on you having a copy of this repository
in `~/.dotfiles`. By sourcing variables from the provided `.bashrc`, you can then use the alias `adopt_home` from anywhere to symlink config packages to your home directory.
```sh
git clone https://github.com/ChionReverie/dotfiles.git $HOME/.dotfiles
source $HOME/.dotfiles/home/.bashrc
adopt_home .
```