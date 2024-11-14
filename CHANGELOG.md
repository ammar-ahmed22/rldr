# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.2]
### Added
- Displays output when command exits
- Displays message if command is already closed and `c` is entered
  + Does not try to "close" it if it's already closed

### Changed
- Display is now colorized with each thing instead of the whole thing being colored
  + Previously, all info messages (everything) was cyan, warnings were yellow, error red
  + Now, only the prefix `[rldr]` is cyan, yellow, or red and the rest is colored individually


## [0.0.1]
### Added
- Running arbitrary commands including subprocesses and chaining
- Watching for input to restart, close and quite
- Support for Unix and Windows systems
- Custom logging with log