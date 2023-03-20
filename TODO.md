# Future Improvements

## Load more logs as you scroll

- [x] Load more entries when reching the bottom of the logs

## Reuse journal pointer to keep position on journal

- Only for log entries, create new one to obtain summary
- Make it thread safe, as the journal handle isn't

## Journal filter

- [x] Priority
- [x] Message text
- [ ] Time range

## Quick filter bar

- [x] Search for a specific message, command

## Dark theme

- Add dark theme
- Add auto-detection to select dark theme

## Break down components

## Refactor to use anyhow

## Publish to AUR

## Invesitgate possibility of flatpak

## Bugs

- Invalid date on most recent block summary
- Log entry: Could not find the field -2
- Loading new entries on scroll might overlap and require other kind of filter or maintain the position on the open journal
