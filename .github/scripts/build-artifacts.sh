#! /bin/bash

set -e

pkgver=$JOURNAL_VERSION

mkdir -p out
rm -rf ./out/*

cd ./src-tauri/target/release/
tar czf 'journal-viewer_'$pkgver'_x86_64.tar.gz' journal-viewer
cd ../../../

cp './src-tauri/target/release/journal-viewer_'$pkgver'_x86_64.tar.gz' ./out/
cp './src-tauri/target/release/bundle/deb/journal-viewer_'$pkgver'_amd64.deb' ./out/
cp ./package/journal-viewer.desktop ./out/
cp ./package/PKGBUILD.tmpl ./out/PKGBUILD

cd out
sha256sumBin=$(sha256sum 'journal-viewer_'$pkgver'_x86_64.tar.gz' | awk '{print $1}')
echo "bin SHA: $sha256sumBin"
sha256sumDesktop=$(sha256sum journal-viewer.desktop | awk '{print $1}')
echo "desktop SHA: $sha256sumDesktop"

sed -i "s/{{pkgver}}/$pkgver/g" PKGBUILD
sed -i "s/{{sha256sumBin}}/$sha256sumBin/g" PKGBUILD
sed -i "s/{{sha256sumDesktop}}/$sha256sumDesktop/g" PKGBUILD

tar czf 'journal-viewer_'$pkgver'_x86_64_AUR.tar.gz' PKGBUILD

rm journal-viewer.desktop
rm PKGBUILD

ls -la -h

cd ..
