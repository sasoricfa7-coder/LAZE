#!/bin/bash
set -e

# 1. Nettoyer les anciens résidus
rm -f laze_disk.img

# 2. Compiler le noyau
cargo build --target x86_64-unknown-uefi --release

# 3. Créer une image disque de 64 Mo
dd if=/dev/zero of=laze_disk.img bs=1M count=64

# 4. Formater en FAT32
mkfs.vfat -F 32 laze_disk.img

# 5. Créer l'arborescence et copier les fichiers
mmd -i laze_disk.img ::/EFI
mmd -i laze_disk.img ::/EFI/BOOT
mcopy -i laze_disk.img target/x86_64-unknown-uefi/release/laze.efi ::/EFI/BOOT/BOOTX64.EFI

# Créer LAZE.TXT en majuscules à la racine
echo "Salutations depuis le VRAI disque de LAZE OS !" > temp_laze.txt
mcopy -i laze_disk.img temp_laze.txt ::/LAZE.TXT
rm temp_laze.txt


# Vérifier ce qu'il y a dans l'image avant de lancer
mdir -i laze_disk.img ::/

# 6. Lancer QEMU sans warnings
qemu-system-x86_64 \
    -bios /usr/share/ovmf/OVMF.fd \
    -drive file=laze_disk.img,format=raw \
    -net none
