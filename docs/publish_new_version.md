# How to publish a new release

- Update package version on tauri.conf.json package.version
- Build release binaries with: `npm run tauri build`
- Create tar.gz file with release binary and obtain the sha256
- Update PKGBUILD pkgver, pkgrel and sha256sums
- Generate SRCINFO Package `makepkg --printsrcinfo > .SRCINFO`
- Copy tar.gz to package folder
- Make package & install `makepkg --install`
- Run `journal-viewer` to test new binary
- Create new release on GH, upload deb and tar.gz, create new tag
- Copy PKGBUILD and .SRCINFO into AUR repo & push changes
