# Journal Viewer

A modern linux desktop application to visualize systemd logs.

## Dark Theme

![Journal Viewer Dark Theme](docs/screenshot-dark.png)

## Light Theme

![Journal Viewer Light Theme](docs/screenshot.png)

## Features

- Summary graph with the latest log entries for the last 5 days or 10k entries
- A quick search to filter messages containing some text (case insensitive).
- A more advanced filter bar where you can search by:
    - Log Priority
    - Date Range
    - Service unit, including init service (systemd)
    - Transport, journal, kernel...
    - System Boot
- Graphical indication for log level
- Expand log entry on selection
- Infinite scrolling.
- Refresh logs button
- Light/Dark theme

## Built with

- Rust
- Systemd Journald
- Tauri
- Vue
- Bootstrap
