# Snappy
[![Actions Status](https://github.com/dodgez/snappy/workflows/Rust/badge.svg)](https://github.com/dodgez/snappy/actions)

Snappy is a basic distributed version control system inspired by `git`.
This software is still in _early_ development.
For more information on the limitations of `snappy`, check out the [issues](https://github.com/dodgez/snappy/issues) tab.

Several notable features are:
- Staging area using `snappy add`
- Commits with messages using `snappy commit <message>`
- Switching between commits using `snappy checkout <commit_hash>`
- Showing the linear history up to _HEAD_ using `snappy log`

Run `snappy` without any arguments or pass the flag `--help` for more information about command-line use.
