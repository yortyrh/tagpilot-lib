#!/usr/bin/env node

/**
 * Benchmark runner script with proper dependency management
 * This script:
 * 1. Prepares benchmark files (yarn prepare-benchmark-files)
 * 2. Links the main package for local development (npm link)
 * 3. Installs benchmark dependencies (cd benchmark && npm install)
 * 4. Runs the benchmark (yarn bench)
 * 5. Cleans up benchmark dependencies (rm -rf benchmark/node_modules)
 * 6. Cleans up benchmark files (rm -rf benchmark-files)
 */

import { execSync } from 'child_process'
import { existsSync, rmSync } from 'fs'
import { join, resolve } from 'path'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = resolve(__filename, '..')
const projectRoot = resolve(__dirname, '..')
const benchmarkDir = join(projectRoot, 'benchmark')
const benchmarkFilesDir = join(projectRoot, 'benchmark-files')
const benchmarkNodeModules = join(benchmarkDir, 'node_modules')

function log(message, emoji = '📝') {
  console.log(`${emoji} ${message}`)
}

function execCommand(command, cwd = projectRoot) {
  try {
    log(`Executing: ${command}`, '⚡')
    execSync(command, {
      cwd,
      stdio: 'inherit',
      encoding: 'utf8',
    })
  } catch (error) {
    log(`Error executing command: ${command}`, '❌')
    log(`Error: ${error.message}`, '❌')
    process.exit(1)
  }
}

function main() {
  try {
    log('Starting benchmark process...', '🚀')

    // Step 1: Prepare benchmark files
    log('Preparing benchmark files...', '📁')
    execCommand('yarn prepare-benchmark-files')

    // Step 2: Link main package for local development
    log('Linking main package for local development...', '🔗')
    execCommand('npm link')

    // Step 3: Install benchmark dependencies
    log('Installing benchmark dependencies...', '📦')
    execCommand('npm install', benchmarkDir)

    // Step 4: Run benchmark
    log('Running benchmark...', '🏃')
    execCommand('yarn bench')

    // Step 5: Clean up benchmark dependencies
    log('Cleaning up benchmark dependencies...', '🧹')
    if (existsSync(benchmarkNodeModules)) {
      rmSync(benchmarkNodeModules, { recursive: true, force: true })
      log('Benchmark dependencies cleaned up', '✅')
    } else {
      log('No benchmark dependencies to clean up', 'ℹ️')
    }

    // Step 6: Clean up benchmark files
    log('Cleaning up benchmark files...', '🗑️')
    if (existsSync(benchmarkFilesDir)) {
      rmSync(benchmarkFilesDir, { recursive: true, force: true })
      log('Benchmark files cleaned up', '✅')
    } else {
      log('No benchmark files to clean up', 'ℹ️')
    }

    log('Benchmark completed successfully!', '🎉')
  } catch (error) {
    log(`Benchmark failed: ${error.message}`, '💥')
    process.exit(1)
  }
}

// Run the script
main()
