# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://github.com/AldaronLau/semver).

## [0.3.0] - 2023-02-19
### Changed
 - Update pasts to 0.14

## [0.2.0] - 2023-01-17
### Added
 - *`web`* feature 
 - `Spawn` trait
 - `LocalSpawner` struct

### Changed
 - Broke proc macro into separate crate
 - Generate same code for all async runtimes
 - Replace attribute parameters with features

### Removed
 - smolscale feature

## [0.1.0] - 2023-01-01
### Added
 - Tokio support
 - Async std support
 - Pasts support
 - Async executor (with Futures Lite) support
 - Smolscale support
 - Futures support
