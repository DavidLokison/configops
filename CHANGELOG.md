# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Remove unnecessary serde_derive dependency

## [1.0.0] - 2026-04-16

### Added

- `Resolver` and `Repository` traits to modularily implement file storage strategies
- `Resolver` implementation for `etcetera::AppStrategy`
- `Error` type to handle IO and parsing failures
- TOML file type support

[Unreleased]: https://github.com/DavidLokison/configops/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/DavidLokison/configops/releases/tag/v1.0.0
