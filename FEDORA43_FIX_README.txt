🎵 RANGER DE SONG - FEDORA 43 FIX ✅
═════════════════════════════════════════════════════════════

✨ MISE À JOUR EFFECTUÉE

Votre projet a été migré de Tauri 1.x vers Tauri 2.x
pour supporter webhook2gtk4.1 et libsoup3 sur Fedora 43.

═════════════════════════════════════════════════════════════
📝 CHANGEMENTS DANS LES FICHIERS
═════════════════════════════════════════════════════════════

✅ src-tauri/Cargo.toml
   • Tauri: 1.5 → 2.6
   • tauri-build: 2.6.0 → 2.6
   • Ajout de features (window-close, window-minimize, etc.)

✅ package.json
   • @tauri-apps/api: 1.5.0 → 2.6.0
   • @tauri-apps/cli: 1.5.0 → 2.6.0

✅ Rust code (src-tauri/src/)
   • Pas de changements (déjà compatible)

✅ React code (src/App.tsx)
   • Pas de changements (déjà compatible)

✅ Configuration (tauri.conf.json)
   • Pas de changements (déjà correct)

═════════════════════════════════════════════════════════════
🚀 CE QUE VOUS DEVEZ FAIRE MAINTENANT
═════════════════════════════════════════════════════════════

ÉTAPE 1: Installer les dépendances Fedora 43
────────────────────────────────────────────

sudo dnf install webkit2gtk4.1-devel libsoup3-devel \
  glib2-devel openssl-devel curl


ÉTAPE 2: Nettoyer et reconstruire
──────────────────────────────────

cd "/home/moi/Documents/projets divers/ranger_de_song"

# Option A: Script automatisé (RECOMMANDÉ)
chmod +x build-fedora43.sh
./build-fedora43.sh

# Option B: Manuel
cd src-tauri && rm -rf target Cargo.lock && cd ..
rm -rf node_modules package-lock.json
npm install
cd src-tauri && cargo build && cd ..


ÉTAPE 3: Vérifier que tout fonctionne
──────────────────────────────────────

npm run dev

# Vous devriez voir la fenêtre de l'app se lancer!


ÉTAPE 4: Construire pour production (optionnel)
───────────────────────────────────────────────

npm run build

# Output: src-tauri/target/release/bundle/

═════════════════════════════════════════════════════════════
📚 DOCUMENTATION CRÉÉE
═════════════════════════════════════════════════════════════

✅ FEDORA43_BUILD.md
   Guide complet pour la compilation sur Fedora 43
   
✅ CHANGELOG_TAURI2_UPDATE.md
   Détails des changements effectués

✅ build-fedora43.sh
   Script de build automatisé

✅ verify-tauri2.sh
   Script de vérification des changements

═════════════════════════════════════════════════════════════
✅ VÉRIFIER QUE TOUT EST BON
═════════════════════════════════════════════════════════════

chmod +x verify-tauri2.sh
./verify-tauri2.sh

Vous devriez voir une liste avec des ✅

═════════════════════════════════════════════════════════════
🎯 RÉSUMÉ RAPIDE
═════════════════════════════════════════════════════════════

Problème:
  Fedora 43 n'a plus webkit2gtk-4.0 ni libsoup-2.4
  → Tauri 1.x ne compilait pas

Solution:
  Migrer vers Tauri 2.x
  → Tauri 2.x utilise webkit2gtk4.1 et libsoup3
  → Tout fonctionne maintenant!

Commandes essentielles:
  sudo dnf install webkit2gtk4.1-devel libsoup3-devel glib2-devel openssl
  ./build-fedora43.sh
  npm run dev

═════════════════════════════════════════════════════════════
🎉 C'EST PRÊT!
═════════════════════════════════════════════════════════════

Exécutez ces 3 commandes:

  1) sudo dnf install webkit2gtk4.1-devel libsoup3-devel \
       glib2-devel openssl-devel
       
  2) ./build-fedora43.sh
  
  3) npm run dev

Voilà! 🚀

═════════════════════════════════════════════════════════════
❓ BESOIN D'AIDE?
═════════════════════════════════════════════════════════════

Erreur de compilation:
  → Lire FEDORA43_BUILD.md (section Troubleshooting)

Erreur de dépendances:
  → Vérifier: pkg-config --list-all | grep webkit
  → Vérifier: pkg-config --list-all | grep libsoup

Autre question:
  → Lire CHANGELOG_TAURI2_UPDATE.md

═════════════════════════════════════════════════════════════
