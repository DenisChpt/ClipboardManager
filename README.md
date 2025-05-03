# ClipboardManager

Un gestionnaire de presse-papiers moderne pour Linux (Wayland) inspiré de l'interface de Windows 11. 

![ClipboardManager Screenshot](screenshot.png)

## Fonctionnalités

- Interface graphique moderne avec coins arrondis et transitions fluides
- Historique du presse-papiers
- Aperçu des éléments copiés (texte, images)
- Recherche dans l'historique
- Support de Wayland
- Mode sombre/clair

## Prérequis

- Rust 1.86.0 ou supérieur
- Environnement Wayland (Sway, GNOME Shell, KDE Plasma)
- Dépendances système (pour les bibliothèques utilisées) :
  - libwayland-dev
  - libxkbcommon-dev
  - pkg-config

## Installation

### Compiler depuis les sources

```bash
git clone https://github.com/votre-nom/clipboard-manager.git
cd clipboard-manager
cargo build --release
```

Le binaire sera disponible dans `target/release/clipboard-manager`.

## Configuration du raccourci clavier

Le gestionnaire de presse-papiers est conçu pour être lancé via un raccourci clavier (Win+V). Voici comment configurer ce raccourci dans différents environnements Wayland :

### Sway

Ajoutez cette ligne à votre fichier de configuration (`~/.config/sway/config`) :

```
bindsym Mod4+v exec /chemin/vers/clipboard-manager
```

### GNOME Shell

Utilisez les paramètres système :

```bash
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clipboard-manager/']"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clipboard-manager/ name "Clipboard Manager"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clipboard-manager/ command "/chemin/vers/clipboard-manager"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clipboard-manager/ binding "<Super>v"
```

### KDE Plasma

1. Allez dans Paramètres système > Raccourcis > Raccourcis personnalisés
2. Ajoutez une nouvelle action personnalisée
3. Donnez-lui un nom comme "Clipboard Manager"
4. Définissez la commande sur le chemin vers l'exécutable
5. Attribuez le raccourci Super+V

## Architecture

ClipboardManager est construit avec les technologies suivantes :

- Rust 1.86.0 - Langage de programmation
- Iced - Bibliothèque UI avec le backend WGPU pour le rendu
- Arboard - Gestion du presse-papiers avec support Wayland
- Tokio - Runtime asynchrone pour les opérations d'I/O
- Sled - Stockage persistant de l'historique du presse-papiers

## Licence

Ce projet est sous licence MIT.