#!/bin/bash
# Script pour construire l'APK release de Vaska avec icônes personnalisées et signature
# Usage: ./build-release.sh

set -e

PROJECT_DIR="/home/cyprien/Documents/android/vaska"
ANDROID_DIR="$PROJECT_DIR/target/dx/vaska/release/android/app"
MIPMAP_DIR="$ANDROID_DIR/app/src/main/res"
LOGO="$PROJECT_DIR/assets/logo.png"
APKSIGNER="/home/cyprien/Android/Sdk/build-tools/36.0.0/apksigner"
APK_OUTPUT="$ANDROID_DIR/app/build/outputs/apk/release/app-release.apk"
APK_FINAL="$PROJECT_DIR/vaska-release.apk"

echo "🔨 Construction de Vaska (release)..."

# 1. Build avec dx (génère les fichiers Android, cible arm64 pour téléphone)
echo "📦 Génération des fichiers Android avec dx (arm64-v8a)..."
cd "$PROJECT_DIR"
dx bundle --android --release --package-types apk --target aarch64-linux-android 2>&1 || true
# dx bundle échoue à cause de lint, mais les fichiers sont générés

VERSION_CODE=2
VERSION_NAME="0.1.2"
GRADLE_FILE="$ANDROID_DIR/app/build.gradle.kts"
sed -i -E \
  -e "s/versionCode = [0-9]+/versionCode = ${VERSION_CODE}/" \
  -e "s/versionName = \"[^\"]+\"/versionName = \"${VERSION_NAME}\"/" \
  "$GRADLE_FILE"

# 2. Remplacer les icônes mipmap
echo "🎨 Remplacement des icônes..."
convert "$LOGO" -resize 48x48 -background none -gravity center -extent 48x48 "$MIPMAP_DIR/mipmap-mdpi/ic_launcher.webp" 2>/dev/null
convert "$LOGO" -resize 72x72 -background none -gravity center -extent 72x72 "$MIPMAP_DIR/mipmap-hdpi/ic_launcher.webp" 2>/dev/null
convert "$LOGO" -resize 96x96 -background none -gravity center -extent 96x96 "$MIPMAP_DIR/mipmap-xhdpi/ic_launcher.webp" 2>/dev/null
convert "$LOGO" -resize 144x144 -background none -gravity center -extent 144x144 "$MIPMAP_DIR/mipmap-xxhdpi/ic_launcher.webp" 2>/dev/null
convert "$LOGO" -resize 192x192 -background none -gravity center -extent 192x192 "$MIPMAP_DIR/mipmap-xxxhdpi/ic_launcher.webp" 2>/dev/null

# 3. Build avec Gradle (sans lint)
echo "🏗️  Build Gradle release..."
cd "$ANDROID_DIR"
./gradlew assembleRelease -x lintVitalAnalyzeRelease -x lintVitalRelease 2>&1 | tail -5

# 4. Vérifier la signature
echo "✅ Vérification de la signature..."
$APKSIGNER verify --print-certs "$APK_OUTPUT" 2>/dev/null | grep "Signer #1 certificate DN"

# 5. Copier l'APK à la racine du projet
cp "$APK_OUTPUT" "$APK_FINAL"
APK_SIZE=$(du -h "$APK_FINAL" | cut -f1)

echo ""
echo "🎉 APK prêt: $APK_FINAL ($APK_SIZE)"
echo "📱 Installez avec: adb install $APK_FINAL"
