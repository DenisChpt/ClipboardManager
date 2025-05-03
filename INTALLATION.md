# Guide d'installation

Ce document détaille les étapes pour installer les dépendances nécessaires à la compilation et l'exécution de ClipboardManager sur différentes distributions Linux.

## Prérequis globaux

- Rust 1.86.0 ou supérieur (`rustup update stable`)
- Environnement Wayland (Sway, GNOME Shell, KDE Plasma)
- Dépendances de développement système (voir ci-dessous selon votre distribution)

## Dépendances par distribution

### Ubuntu/Debian

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libwayland-dev \
    libxkbcommon-dev \
    libx11-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxcb-render0-dev \
    libxcb-composite0-dev \
    libxcb-sync-dev \
    libegl1-mesa-dev
```

### Fedora

```bash
sudo dnf install -y \
    gcc \
    pkg-config \
    wayland-devel \
    libxkbcommon-devel \
    libX11-devel \
    libxcb-devel \
    mesa-libEGL-devel
```

### Arch Linux

```bash
sudo pacman -S --needed \
    base-devel \
    pkg-config \
    wayland \
    libxkbcommon \
    libx11 \
    libxcb \
    mesa
```

## Installation depuis les sources

1. Clonez le dépôt
   ```bash
   git clone https://github.com/votre-nom/clipboard-manager.git
   cd clipboard-manager
   ```

2. Compilation
   ```bash
   cargo build --release
   ```

3. Installation (optionnel)
   ```bash
   sudo cp target/release/clipboard-manager /usr/local/bin/
   ```

## Configuration du raccourci clavier

Voir le fichier README.md pour les instructions détaillées de configuration du raccourci clavier selon votre environnement Wayland.

## Problèmes connus

### Fenêtre invisible sous Wayland

Sous Wayland, les fenêtres ne s'affichent pas tant qu'elles n'ont pas été rendues. Si la fenêtre n'apparaît pas, essayez d'appuyer sur des touches ou de déplacer la souris dans la zone où elle devrait apparaître.

### Crash avec l'erreur "XDG_RUNTIME_DIR not set"

Si vous obtenez cette erreur, assurez-vous que la variable d'environnement XDG_RUNTIME_DIR est correctement définie :

```bash
export XDG_RUNTIME_DIR=/run/user/$(id -u)
```

### Permissions du presse-papiers sous Wayland

Certains compositors Wayland peuvent avoir des restrictions sur l'accès au presse-papiers. Si vous rencontrez des problèmes, vérifiez les autorisations de votre compositor.