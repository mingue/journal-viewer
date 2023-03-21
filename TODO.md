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

- [x] Invalid date on most recent block summary, some entries don't have a valid date, so we just filter them out from summary
- [x] Log entry: Could not find the field -2, time field is not found, change message to display field and the JournalError code
- [ ] Loading new entries on scroll might overlap and require other kind of filter or maintain the position on the open journal
