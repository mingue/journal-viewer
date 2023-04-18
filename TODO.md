# Future Improvements

## Load more logs as you scroll

- [x] Load more entries when reching the bottom of the logs

## Reuse journal pointer to keep position on journal

- [x] Allow async queries
- [x] Reuse the journal pointer between queries

## Journal filter

- [x] Priority
- [x] Message text
- [x] Unit
- [ ] Time range
- [x] Transports

## Quick filter bar

- [x] Search for a specific message, command

## Dark theme

- [x] Add dark theme
- [x] Add auto-detection to select dark theme

## Break down components

- [x] Break down vue components

## Refactor to use anyhow

- [ ] Use anyhow crate for app errors

## Publish to AUR

- [x] Published in <https://aur.archlinux.org/packages/journal-viewer-bin>

## Automate relase process

- [x] Document build & publish process
- [ ] Automate process, partially done, missing aur upload

## Investigate possibility of flatpak

## Bugs

- [x] Invalid date on most recent block summary, some entries don't have a valid date, so we just filter them out from summary
- [x] Log entry: Could not find the field -2, time field is not found, change message to display field and the JournalError code
- [x] Loading new entries on scroll might overlap and require other kind of filter or maintain the position on the open journal
