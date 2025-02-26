# Phase 1.0: Install Linux (Arch)

Be sure to closely examine the [Installation Guide on Arch Wiki](https://wiki.archlinux.org/title/Installation_guide),
in case I missed anything of importance.

Partitions on GPT (customize to the needs of your system)

- Boot: EFI system partition. Format to fat32 --> mount on `/mnt/boot`
- Swap: linux Swap. --> Mount using `swapon /dev/___`
- Root: linux x86-64 root. Format to ext4 --> mount on `/mnt`
- Home: linux filesystem. Format to ext4 --> mount on `/mnt/home/`

Be sure to save your sftab.

```sh
genfstab -U /mnt >> /mnt/etc/fstab
```

From here, you can install the linux kernel and Arch.

```sh
pacstrap -K /mnt base linux linux-firmware amd-ucode intel-ucode
```

You can exclude the ucode package for the processor you do not use, but it is
safe to have both.

From here, enter arch's root directory.

```
arch-chroot /mnt
```

Then we can use pacman to activate other necessary packages for networking.

```sh
pacman --sync dhcpcd iwd
systemctl enable dhcpcd iwd
```

My laptop also relies on `broadcom-wl` to connect via wifi.

## Install Grub bootloader

```sh
pacman --sync grub efibootmgr
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=GRUB
grub-mkconfig -o /boot/grub/grub.cfg
```

## Other chores

Follow the installation guide above for the following tasks:

- Set localization information
- Set time locale
- Set hostname
- Set root password

## Follow up

From this point, your network should be set up such that you can boot into
Linux and start setting up your system.
