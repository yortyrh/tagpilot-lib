# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.27] - 2025-09-23 (Current)

### Fixed

- Fixed bug in `validatePath` helper where 'Access denied' string was treated as valid path
- Improved error handling in path validation for better security
- Enhanced error messages for invalid paths with more descriptive feedback

### Added

- Support for multiple image types in audio tags
- New `allImages` field for handling multiple images in tags
- Improved image handling documentation and examples

## [1.0.26] - 2025

### Fixed

- Fixed issue with `clearTags` failing with "Invalid argument (os error 22)"
- Fixed issue with `writeTags` failing with "Invalid argument (os error 22)"
- Updated native binding package versions

### Added

- Enhanced audio image handling with new `AudioImageType` enum
- Added unit tests for all image types in audio tags

## [1.0.25] - 2025

### Security

- Implemented path validation in example scripts for security enhancement
- Added security checks for file handling

### Added

- Added benchmark workflow for performance tracking
- Enhanced CI workflow with automated benchmarking

## [1.0.24] - 2025

### Fixed

- Updated native binding version check to 1.0.23
- Implemented path traversal validation in example scripts

## [1.0.21] - [1.0.22] - 2025

### Added

- Enhanced CI workflow with conditional PR comments
- Added benchmark results and performance tracking

## [1.0.20] - 2025

### Added

- Implemented benchmark comparison with music-metadata
- Enhanced security utilities for file handling

## [1.0.17] - [1.0.19] - 2025

### Added

- Added code coverage reporting
- Enhanced test coverage with additional edge cases
- Added Debug and PartialEq implementations for Image struct

## [1.0.14] - [1.0.16] - 2025

### Added

- Added Rust unit testing
- Added GitHub Packages publishing support
- Enhanced error handling in Rust files

## [1.0.13] - 2025

### Added

- Added comprehensive unit testing
- Improved code readability and consistency
- Enhanced WASI support

## [1.0.10] - [1.0.12] - 2025

### Added

- Support for multiple artists in metadata
- Enhanced cover image handling
- Improved documentation and examples

## [1.0.7] - [1.0.9] - 2025

### Added

- Support for Windows ARM64
- Enhanced audio tag functionality
- Improved error handling
- Added new Image interface

### Changed

- Updated package name to `@yortyrh/tagpilot-lib`
- Refactored metadata structure for better consistency

## [1.0.4] - [1.0.6] - 2025

### Added

- Support for armv7-linux-androideabi
- Support for wasm32-wasip1-threads
- Enhanced CI workflow

## [1.0.1] - [1.0.3] - 2025

### Changed

- Package rebranded to `@yortyrh/tagpilot-lib`
- Updated license and documentation
- Improved error handling

### Added

- Buffer-based operations for tags and cover images
- Asynchronous API support
- Clear tags functionality

## [1.0.0] - 2025

### Added

- Initial release
- Basic audio metadata reading and writing
- Cover image support
- Cross-platform support
- WebAssembly support
