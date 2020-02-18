# Snappy
[![Actions Status](https://github.com/dodgez/snappy/workflows/Rust/badge.svg)](https://github.com/dodgez/snappy/actions)

Snappy is a basic distributed version control system inspired by `git`.
This software is still in _early_ development.
For more information on the limitations of `snappy`, check out the [issues](https://github.com/dodgez/snappy/issues) tab.

Several notable features are:
- Create a snappy repository using `snappy init`
- Add to the staging area using `snappy add <file or directory>`
- Create commits using `snappy commit -m <message>`
- Switch between commits (or branches) using `snappy checkout <branch or commit_hash>`
- Create a branch using `snappy branch <new_branch_name>`
- Fast-forward merge a branch (or a child commit) using `snappy merge <child branch or commit>`
- Show the linear history up to _HEAD_ using `snappy log`

Run `snappy` without any arguments or pass the flag `--help` for more information about command-line use.
