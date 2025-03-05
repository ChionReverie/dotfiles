# Phase 1.1: Setup System

Once you have successfully booted into Arch Linux, you can begin setting up
the system to your needs.

Remember that you can always boot into the livedisk for repairs (for example
if your network is not behaving).

## Some Necessary Tools

```sh
pacman --sync nano sudo git wget man-db man-pages texinfo
```

Edit the sudoers config with `EDITOR=nano visudo`. There is a line somewhere
in the file that starts with `# %wheel ALL=(ALL:ALL) ALL`. Remove the `# ` at
the beginning to uncomment the line, thus allowing users in the `wheel` group
to use the sudo command.

This guide will reference files from my dotfiles git repo.

```sh
git clone https://github.com/ChionReverie/dotfiles.git ~/.dotfiles
```

## Advanced Networking

```sh
pacman --sync openssh zerotier-one
sudo cp --recursive \
    ~/.dotfiles/root/etc/ssh/sshd_config.d/* \
    /etc/ssh/sshd-config.d/
systemctl enable sshd zerotier-one
```

For systems that will not be part of a zerotier network, you can of course  
skip installing `zerotier-one`.


