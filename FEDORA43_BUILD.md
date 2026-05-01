# Fedora 43 Build Guide - Ranger de Song

## ✅ Changements Effectués

Votre projet a été mis à jour pour **Tauri 2.x**, qui supporte nativement **webkit2gtk4.1** et **libsoup3** (les versions disponibles sur Fedora 43).

### Changements dans les fichiers:

| Fichier | Changement |
|---------|-----------|
| `Cargo.toml` | Tauri: 1.5 → **2.6** |
| `package.json` | @tauri-apps/api: 1.5 → **2.6** |
| `package.json` | @tauri-apps/cli: 1.5 → **2.6** |

Le code Rust et React est **déjà compatible** avec Tauri 2.x.

---

## 🚀 Installation sur Fedora 43

### Étape 1: Installer les dépendances système

```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install webkit2gtk4.1-devel libsoup3-devel glib2-devel openssl-devel curl
```

Si vous avez des erreurs avec les noms de paquets, utilisez:
```bash
dnf search webkit | grep devel
dnf search libsoup | grep devel
```

### Étape 2: Vérifier Node.js et Rust

```bash
# Installer Node.js si nécessaire
curl -fsSL https://deb.nodesource.com/setup_18.x | bash
# (ou depuis https://nodejs.org/)

# Installer Rust si nécessaire
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Étape 3: Nettoyer et reconstruire

```bash
cd "/home/moi/Documents/projets divers/ranger_de_song"

# Option A: Utiliser le script de build
chmod +x build-fedora43.sh
./build-fedora43.sh

# Option B: Faire manuellement
cd src-tauri
rm -rf target Cargo.lock
cd ..
rm -rf node_modules package-lock.json

npm install
cd src-tauri && cargo build && cd ..
```

### Étape 4: Démarrer le serveur de développement

```bash
npm run dev
```

---

## ✨ Pourquoi Tauri 2.x?

**Tauri 1.x** était lié aux anciennes versions de webkit et libsoup:
- ❌ webkit2gtk-4.0 (obsolète sur Fedora 43)
- ❌ libsoup-2.4 (obsolète sur Fedora 43)

**Tauri 2.x** utilise les dépendances système directement:
- ✅ webkit2gtk4.1 (disponible sur Fedora 43)
- ✅ libsoup3.0 (disponible sur Fedora 43)
- ✅ Meilleure performance
- ✅ Support moderne de web standards

---

## 🔧 Variables d'environnement (au besoin)

Si vous avez toujours des problèmes de build:

```bash
# Spécifier les chemins de webkit2gtk
export PKG_CONFIG_PATH="/usr/lib64/pkgconfig:$PKG_CONFIG_PATH"

# Ou pour les dépendances de développement
export LD_LIBRARY_PATH="/usr/lib64:$LD_LIBRARY_PATH"

# Puis reconstruire
cd src-tauri && cargo build && cd ..
```

---

## 🐛 Troubleshooting

### Erreur: "webkit2gtk not found"
```bash
# Vérifier que webkit2gtk4.1-devel est installé
dnf list installed | grep webkit

# Si absent, installer
sudo dnf install webkit2gtk4.1-devel
```

### Erreur: "libsoup-3.0 not found"
```bash
# Vérifier que libsoup3 est installé
dnf list installed | grep libsoup

# Si absent, installer
sudo dnf install libsoup3-devel
```

### Erreur: "cannot find -lglib-2.0"
```bash
# Installer les développement headers
sudo dnf install glib2-devel
```

### Cargo refuse de compiler
```bash
# Nettoyer complètement
cd src-tauri
rm -rf target Cargo.lock ~/.cargo/registry/cache
cd ..

# Reconstruire
cd src-tauri && cargo build && cd ..
```

### npm: command not found
```bash
# Installer Node.js depuis https://nodejs.org/
# Ou via dnf
sudo dnf install npm
```

---

## ✅ Vérification des versions

Une fois installé, vérifiez:

```bash
# Vérifier que webkit2gtk4.1 est utilisé
pkg-config --cflags --libs webkit2gtk-4.1

# Vérifier que libsoup3 est utilisé  
pkg-config --cflags --libs libsoup-3.0

# Vérifier Rust
rustc --version

# Vérifier Node
node --version && npm --version
```

---

## 📱 Construire pour production

Une fois que le développement fonctionne:

```bash
npm run build
```

Les fichiers de distribution seront dans:
- `src-tauri/target/release/bundle/appimage/` (Linux AppImage)
- `src-tauri/target/release/bundle/rpm/` (RPM pour Fedora)

---

## 🎉 Prêt!

Vous pouvez maintenant développer et construire sur Fedora 43:

```bash
npm run dev    # Développement avec hot-reload
npm run build  # Production build
```

**Bon développement!** 🚀🎵

---

*Guide mis à jour pour Tauri 2.6 et Fedora 43 (webkit2gtk4.1 + libsoup3)*
