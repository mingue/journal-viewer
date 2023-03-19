# Future Improvements

## Reuse journal pointer to keep position on journal

- Only for log entries, create new one to obtain summary
- Make it thread safe, as the journal handle isn't

## Load more logs as you scroll

- Load more entries when reching the bottom of the logs

## Journal filter

- [x] Priority
- [x] Message text
- [ ] Time range

## Quick filter bar

- Search in loaded entries for a specific message, command

## Break down components

## Refactor to use anyhow

## Bugs

- Invalid date on most recent block summary
- Log entry: Could not find the field -2
