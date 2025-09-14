# Scripts

This directory contains utility scripts for the tagpilot-lib project.

## Benchmark Scripts

### `run-benchmark.mjs` (Cross-Platform)

Cross-platform benchmark runner with proper dependency management:

```bash
node scripts/run-benchmark.mjs
# or
yarn bench:clean
```

**What it does:**

1. Prepares benchmark files (`yarn prepare-benchmark-files`)
2. Links the main package for local development (`npm link`)
3. Installs benchmark dependencies (`cd benchmark && npm install`)
4. Runs the benchmark (`yarn bench`)
5. Cleans up benchmark dependencies (`rm -rf benchmark/node_modules`)
6. Cleans up benchmark files (`rm -rf benchmark-files`)

### Legacy Scripts

- `run-benchmark.sh` - Unix/Linux/macOS shell script (legacy)
- `run-benchmark.bat` - Windows batch script (legacy)

## Other Scripts

- `convert-from-mp3.mjs` - Convert audio files between formats
- `generate-test-data.mjs` - Generate test data for unit tests
- `generate-npm-packages.mjs` - Generate platform-specific npm packages
- `prepare-benchmark-files.mjs` - Prepare audio files for benchmarking

## Usage

All scripts can be run from the project root:

```bash
# Run benchmark with cleanup
yarn bench:clean

# Run benchmark without cleanup (keeps dependencies)
yarn bench

# Run individual scripts
node scripts/convert-from-mp3.mjs music test-files
node scripts/generate-test-data.mjs
node scripts/generate-npm-packages.mjs
```
