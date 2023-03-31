#! /bin/bash

set -e

pkgver=${GITHUB_REF_NAME:1}
echo "Publishing version: $pkgver"

mkdir -p out
rm -rf ./out/*

cd ./src-tauri/target/release/
tar czf 'journal-viewer_'$pkgver'_x86_64.tar.gz' journal-viewer
cd ../../../

cp './src-tauri/target/release/journal-viewer_'$pkgver'_x86_64.tar.gz' ./out/
cp './src-tauri/target/release/bundle/deb/journal-viewer_'$pkgver'_amd64.deb' ./out/
cp ./package/journal-viewer.desktop ./out/
cp ./package/PKGBUILD-bin.tmpl ./out/PKGBUILD-bin
cp ./package/PKGBUILD-src.tmpl ./out/PKGBUILD-src

cd out
sha256sumBin=$(sha256sum 'journal-viewer_'$pkgver'_x86_64.tar.gz' | awk '{print $1}')
echo "bin SHA: $sha256sumBin"
sha256sumDesktop=$(sha256sum journal-viewer.desktop | awk '{print $1}')
echo "desktop SHA: $sha256sumDesktop"

sed -i "s/{{pkgver}}/$pkgver/g" PKGBUILD-bin
sed -i "s/{{pkgver}}/$pkgver/g" PKGBUILD-src
sed -i "s/{{sha256sumBin}}/$sha256sumBin/g" PKGBUILD-bin
#sha256sumSrc should be replaced before pushing to AUR as we don't have tar.gz of src code yet
sed -i "s/{{sha256sumDesktop}}/$sha256sumDesktop/g" PKGBUILD-bin
sed -i "s/{{sha256sumDesktop}}/$sha256sumDesktop/g" PKGBUILD-src

tar czf 'journal-viewer_'$pkgver'_x86_64_AUR.tar.gz' PKGBUILD-bin PKGBUILD-src

rm journal-viewer.desktop
rm PKGBUILD-bin
rm PKGBUILD-src

ls -la -h

cd ..
