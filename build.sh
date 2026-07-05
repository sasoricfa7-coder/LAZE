#!/bin/bash
set -e

# 1. Nettoyer
rm -rf esp/

# 2. Compiler pour l'architecture UEFI 64 bits standard
cargo build --target x86_64-unknown-uefi --release

# 3. Créer l'arborescence de fichiers requise par la norme UEFI
mkdir -p esp/EFI/BOOT

# 4. Copier le binaire UEFI généré au bon endroit sous le nom standardisé
cp target/x86_64-unknown-uefi/release/laze.efi esp/EFI/BOOT/BOOTX64.EFI

# 5. Lancer QEMU avec la syntaxe de lecteur FAT standardisée
qemu-system-x86_64 \
    -bios /usr/share/ovmf/OVMF.fd \
    -drive file=fat:rw:esp,format=raw \
    -net none
