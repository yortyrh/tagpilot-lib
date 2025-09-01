# NPM Artifacts Setup

This document describes the npm artifacts setup for the `@yortyrh/tagpilot-lib` project, which enables cross-platform distribution of native binaries and WebAssembly modules.

## Overview

The npm artifacts system allows publishing platform-specific packages that contain the appropriate native binaries for each target platform. This ensures that users get the correct binary for their system when installing the package. The system also supports WebAssembly (WASM) for browser environments.

## Directory Structure

The npm artifacts are organized in the following structure:

```
npm/
├── android-arm-eabi/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.android-arm-eabi.node
├── android-arm64/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.android-arm64.node
├── darwin-arm64/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.darwin-arm64.node
├── darwin-x64/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.darwin-x64.node
├── freebsd-x64/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.freebsd-x64.node
├── linux-arm-gnueabihf/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.linux-arm-gnueabihf.node
├── linux-arm64-gnu/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.linux-arm64-gnu.node
├── linux-arm64-musl/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.linux-arm64-musl.node
├── linux-x64-gnu/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.linux-x64-gnu.node
├── linux-x64-musl/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.linux-x64-musl.node
├── win32-arm64-msvc/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.win32-arm64-msvc.node
├── win32-ia32-msvc/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.win32-ia32-msvc.node
├── win32-x64-msvc/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.win32-x64-msvc.node
├── wasm32-wasip1-threads/
│   ├── README.md
│   ├── package.json
│   ├── index.js
│   ├── index.d.ts
│   └── tagpilot_lib.wasm32-wasip1-threads.wasm
└── wasm32-wasi/
    ├── README.md
    ├── package.json
    ├── index.js
    ├── index.d.ts
    └── tagpilot_lib.wasi-browser.js
```

## Supported Platforms

The following platforms are supported:

### Android

- **android-arm-eabi**: Android ARM EABI
- **android-arm64**: Android ARM64

### macOS

- **darwin-arm64**: macOS ARM64 (Apple Silicon)
- **darwin-x64**: macOS x64 (Intel)

### Linux

- **linux-arm-gnueabihf**: Linux ARM GNU EABI HF
- **linux-arm64-gnu**: Linux ARM64 GNU
- **linux-arm64-musl**: Linux ARM64 Musl
- **linux-x64-gnu**: Linux x64 GNU
- **linux-x64-musl**: Linux x64 Musl

### Windows

- **win32-arm64-msvc**: Windows ARM64 MSVC
- **win32-ia32-msvc**: Windows IA32 MSVC
- **win32-x64-msvc**: Windows x64 MSVC

### FreeBSD

- **freebsd-x64**: FreeBSD x64

### WebAssembly

- **wasm32-wasip1-threads**: WebAssembly System Interface with threads support
- **wasm32-wasi**: WebAssembly System Interface for browser environments

## Package Configuration

Each platform-specific package includes:

### package.json

- Platform-specific package name: `@yortyrh/tagpilot-lib-{platform}`
- Platform-specific description
- OS, CPU, and libc specifications
- All other metadata from the main package

### README.md

- Platform-specific documentation
- Installation instructions
- Usage examples
- Supported formats

### Binary Files

- **index.js**: Main JavaScript entry point
- **index.d.ts**: TypeScript definitions
- **tagpilot_lib.{platform}.node**: Native binary for the specific platform
- **tagpilot_lib.{platform}.wasm**: WebAssembly module (for WASM platforms)
- **tagpilot_lib.wasi-browser.js**: Browser-compatible WASI implementation

## Usage

### Creating NPM Directories

```bash
yarn create-npm-dirs
```

### Generating NPM Packages

```bash
yarn generate-npm-packages
```

This command creates the npm directory structure and generates platform-specific package.json files.

### Copying Artifacts

```bash
yarn artifacts
```

This command copies the appropriate native binaries and main files to each npm directory. It runs automatically after `yarn generate-npm-packages`.

### Publishing

```bash
yarn prepublishOnly
```

This command prepares the packages for publishing to npm.

## Artifacts Directory

The artifacts directory contains the native binaries and WebAssembly modules organized by build target:

```
artifacts/
├── bindings-x86_64-apple-darwin/
│   └── tagpilot_lib.darwin-x64.node
├── bindings-aarch64-apple-darwin/
│   └── tagpilot_lib.darwin-arm64.node
├── bindings-x86_64-pc-windows-msvc/
│   └── tagpilot_lib.win32-x64-msvc.node
├── bindings-wasm32-wasip1-threads/
│   └── tagpilot_lib.wasm32-wasip1-threads.wasm
├── bindings-wasm32-wasi/
│   └── tagpilot_lib.wasi-browser.js
└── ...
```

## GitHub Actions Integration

The npm artifacts system is integrated with GitHub Actions workflows:

1. **CI Workflow**: Builds artifacts for all platforms
2. **Release Workflow**: Publishes platform-specific packages
3. **PR Workflow**: Tests artifacts on multiple platforms

## Benefits

1. **Cross-platform compatibility**: Users get the correct binary for their platform
2. **WebAssembly support**: Browser environments can use WASM modules
3. **Automatic platform detection**: npm automatically selects the right package
4. **Reduced package size**: Each package only contains the necessary binary
5. **Better performance**: No need to download unnecessary binaries
6. **Simplified installation**: Users don't need to worry about platform compatibility
7. **Universal compatibility**: Supports both Node.js and browser environments

## Configuration

### Package Configuration

The main `package.json` includes:

```json
{
  "browser": "browser.js",
  "files": ["index.d.ts", "index.js", "browser.js"],
  "napi": {
    "binaryName": "tagpilot_lib",
    "targets": [
      "x86_64-pc-windows-msvc",
      "x86_64-apple-darwin",
      "x86_64-unknown-linux-gnu",
      "x86_64-unknown-linux-musl",
      "aarch64-unknown-linux-gnu",
      "i686-pc-windows-msvc",
      "armv7-unknown-linux-gnueabihf",
      "aarch64-apple-darwin",
      "aarch64-linux-android",
      "x86_64-unknown-freebsd",
      "aarch64-unknown-linux-musl",
      "aarch64-pc-windows-msvc",
      "armv7-linux-androideabi",
      "wasm32-wasip1-threads"
    ]
  }
}
```

### Scripts

- **`preartifacts`**: Automatically runs `yarn generate-npm-packages` before artifacts
- **`artifacts`**: Uses the official NAPI-RS CLI to copy artifacts
- **`generate-npm-packages`**: Creates npm directory structure and package.json files

### Environment Variables

Required secrets:

- `NPM_TOKEN`: npm registry authentication token
- `GITHUB_TOKEN`: GitHub API token (automatically provided)

### Workflow Configuration

Key configuration options:

- **Matrix builds**: Multiple platforms/versions including WASM targets
- **Caching**: npm and cargo cache for faster builds
- **Retention**: Artifact retention periods
- **Triggers**: Workflow trigger conditions

## Troubleshooting

### Missing Artifacts

If some artifacts are missing, the `yarn artifacts` command will show warnings. This is normal when not all platforms have been built locally.

### Binary Name Mismatch

Ensure the `binaryName` in `package.json` matches the actual binary file names. The current configuration uses `tagpilot_lib`.

### WASM Build Issues

For WebAssembly targets, ensure:

- Rust toolchain supports WASM targets
- WASM targets are properly configured in `Cargo.toml`
- Browser compatibility is tested

### Platform Support

To add support for a new platform:

1. Add the platform to the `targets` array in `package.json`
2. Add the platform to the build matrix in GitHub Actions
3. Update the `generate-npm-packages.js` script if needed
4. Test the new platform

### Automation Features

The new setup includes several automation features:

- **Pre-artifacts hook**: Automatically generates npm packages before copying artifacts
- **Dynamic package generation**: Creates platform-specific package.json files automatically
- **NAPI-RS integration**: Uses the official CLI for artifact management
- **WASM support**: Automatically handles WebAssembly targets

## Workflow

### Complete Artifacts Process

1. **Setup**: Run `yarn create-npm-dirs` to create the basic structure
2. **Generation**: Run `yarn generate-npm-packages` to create platform-specific packages
3. **Build**: Build artifacts for all platforms using `yarn build --platform --release`
4. **Copy**: Run `yarn artifacts` to copy artifacts to npm directories
5. **Publish**: Use `yarn prepublishOnly` to prepare for npm publishing

### Automation

The `yarn artifacts` command now automatically:

- Runs `yarn generate-npm-packages` first (via preartifacts hook)
- Uses the official NAPI-RS CLI for artifact management
- Handles both native binaries and WebAssembly modules
- Creates platform-specific package.json files dynamically

## References

- [NAPI-RS Documentation](https://napi.rs/docs/cli/artifacts)
- [npm Package Configuration](https://docs.npmjs.com/cli/v8/configuring-npm/package-json)
- [GitHub Actions Artifacts](https://docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts)
- [WebAssembly System Interface](https://wasi.dev/)
