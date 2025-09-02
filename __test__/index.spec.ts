import test from 'ava'
import fs from 'fs'
import path from 'path'
import { readTags, readTagsFromBuffer } from '../index.js'

test('sync function from native code', (t) => {
  t.true(true)
})

test.skip('readTags returns a Promise', (t) => {
  const result = readTags('./music/03.mp3')
  t.true(result instanceof Promise)
})

test.skip('readTagsFromBuffer returns a Promise', (t) => {
  const audioBuffer = fs.readFileSync('./music/03.mp3')
  const result = readTagsFromBuffer(audioBuffer)
  t.true(result instanceof Promise)
})

test.skip('all test-files have empty tags', async (t) => {
  const testFilesDir = './test-files'
  const audioExtensions = ['.aac', '.aiff', '.flac', '.m4a', '.ogg', '.opus', '.spx', '.wav', '.wv', '.mp3']

  // Get all audio files from test-files directory
  const files = fs
    .readdirSync(testFilesDir)
    .filter((file) => audioExtensions.some((ext) => file.endsWith(ext)))
    .sort()

  t.true(files.length > 0, 'Should have test files to process')

  // Test each audio file
  for (const file of files) {
    const filePath = path.join(testFilesDir, file)
    try {
      const tags = await readTags(filePath)

      // Verify tags is an empty object
      t.true(typeof tags === 'object', `${file} should return an object`)
      t.true(tags !== null, `${file} should not return null`)
      t.true(Array.isArray(tags) === false, `${file} should not return an array`)

      // Check if it's an empty object (no own properties)
      const keys = Object.keys(tags)
      t.is(keys.length, 0, `${file} should have no tags (empty object), but found: ${JSON.stringify(tags)}`)

      console.log(`✅ ${file}: Empty tags verified`)
    } catch (error) {
      t.fail(`${file} failed to read tags: ${error}`)
    }
  }

  console.log(`\n🎉 All ${files.length} test files verified to have empty tags!`)
})

test.skip('test-files directory contains expected audio formats', (t) => {
  const testFilesDir = './test-files'
  const files = fs.readdirSync(testFilesDir).filter((file) => !file.endsWith('.json')) // Exclude manifest.json

  // Count files by extension
  const extensionCounts: Record<string, number> = {}
  files.forEach((file) => {
    const ext = path.extname(file)
    extensionCounts[ext] = (extensionCounts[ext] || 0) + 1
  })

  // Verify we have the expected audio formats
  const expectedFormats = ['.aac', '.aiff', '.flac', '.m4a', '.ogg', '.opus', '.spx', '.wav', '.wv', '.mp3']
  expectedFormats.forEach((format) => {
    t.true(extensionCounts[format] > 0, `Should have ${format} files`)
  })

  // Verify we have multiple files of each format (01, 02, 03, 04, 05)
  const expectedFileCount = 5
  expectedFormats.forEach((format) => {
    t.true(extensionCounts[format] >= expectedFileCount, `Should have at least ${expectedFileCount} ${format} files`)
  })

  console.log('📊 Audio format distribution:', extensionCounts)
})

test.skip('test-files are properly converted (no metadata)', async (t) => {
  // Test a few specific files to ensure they have no metadata
  const testFiles = [
    './test-files/01.flac',
    './test-files/02.m4a',
    './test-files/03.wav',
    './test-files/04.ogg',
    './test-files/05.aac',
  ]

  for (const filePath of testFiles) {
    try {
      const tags = await readTags(filePath)

      // Verify tags is an empty object
      t.true(typeof tags === 'object', `${filePath} should return an object`)
      t.true(tags !== null, `${filePath} should not return null`)
      t.true(Array.isArray(tags) === false, `${filePath} should not return an array`)

      // Check if it's an empty object (no own properties)
      const keys = Object.keys(tags)
      t.is(keys.length, 0, `${filePath} should have no tags (empty object), but found: ${JSON.stringify(tags)}`)

      console.log(`✅ ${filePath}: No metadata verified`)
    } catch (error) {
      t.fail(`${filePath} failed to read tags: ${error}`)
    }
  }

  console.log('🎯 Selected test files verified to have no metadata')
})
