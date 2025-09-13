import { Bench } from 'tinybench'
import fs from 'fs/promises'
import path from 'path'
import { readTags } from '../index.js'
import { parseFile } from 'music-metadata'

// Test data setup
const BENCHMARK_FILES_DIR = path.join(process.cwd(), 'benchmark-files')
const SUPPORTED_FORMATS = ['.mp3', '.flac', '.ogg', '.opus', '.aiff']
let testFiles: string[] = []

async function setupTestData() {
  console.log('Setting up test data...')

  try {
    const files = await fs.readdir(BENCHMARK_FILES_DIR)
    testFiles = files
      .filter((file) => SUPPORTED_FORMATS.some((format) => file.endsWith(format)))
      .map((file) => path.join(BENCHMARK_FILES_DIR, file))

    console.log(`Found ${testFiles.length} test files`)
  } catch (error) {
    console.error('Failed to setup test data:', (error as Error).message)
    process.exit(1)
  }
}

async function runBenchmark() {
  await setupTestData()

  const bench = new Bench({ time: 2000 }) // 2 seconds per test

  // Tagpilot-lib: Read tags from file
  bench.add('tagpilot-lib: readTags', async () => {
    for (const filePath of testFiles) {
      try {
        await readTags(filePath)
      } catch (error) {
        // Ignore errors for unsupported formats
        console.error('Error reading file:', (error as Error).message)
      }
    }
  })

  // music-metadata: Read tags from file
  bench.add('music-metadata: parseFile', async () => {
    for (const filePath of testFiles) {
      try {
        await parseFile(filePath)
      } catch (error) {
        // Ignore errors for unsupported formats
        console.error('Error parsing file:', (error as Error).message)
      }
    }
  })

  // Format-specific benchmarks
  for (const format of ['.mp3', '.flac', '.ogg']) {
    const formatFiles = testFiles.filter((f) => f.endsWith(format))
    if (formatFiles.length === 0) continue

    bench.add(`tagpilot-lib: readTags (${format})`, async () => {
      for (const filePath of formatFiles) {
        try {
          await readTags(filePath)
        } catch (error) {
          // Ignore errors
          console.error('Error reading file:', (error as Error).message)
        }
      }
    })

    bench.add(`music-metadata: parseFile (${format})`, async () => {
      for (const filePath of formatFiles) {
        try {
          await parseFile(filePath)
        } catch (error) {
          // Ignore errors
          console.error('Error parsing file:', (error as Error).message)
        }
      }
    })
  }

  console.log('Running benchmarks...')
  console.log('This may take a few minutes...\n')

  await bench.run()

  console.log('\n=== BENCHMARK RESULTS ===\n')
  console.table(bench.table())

  // Calculate performance ratios
  const results = bench.table()
  const tagpilotReadFile = results.find((r) => r?.name === 'tagpilot-lib: readTags')
  const musicMetadataReadFile = results.find((r) => r?.name === 'music-metadata: parseFile')

  if (tagpilotReadFile && musicMetadataReadFile && tagpilotReadFile.average && musicMetadataReadFile.average) {
    const ratio = Number(musicMetadataReadFile.average) / Number(tagpilotReadFile.average)
    console.log(`\n=== PERFORMANCE COMPARISON ===`)
    console.log(
      `tagpilot-lib is ${ratio.toFixed(2)}x ${ratio > 1 ? 'faster' : 'slower'} than music-metadata for reading tags`,
    )
  }
}

runBenchmark().catch(console.error)
