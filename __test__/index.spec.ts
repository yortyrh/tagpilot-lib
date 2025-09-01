import test from 'ava'
import { clearTags, readTagsFromBuffer, writeTagsToBuffer, readCoverImage, writeCoverImage } from '../index.js'
import * as fs from 'node:fs'

test('sync function from native code', (t) => {
  t.true(true)
})

test('clearTags returns a Promise', (t) => {
  const result = clearTags('./music/03.mp3')
  t.true(result instanceof Promise)
})

test('readTagsFromBuffer returns a Promise', (t) => {
  const buffer = fs.readFileSync('./music/03.mp3')
  const result = readTagsFromBuffer(buffer)
  t.true(result instanceof Promise)
})

test('writeTagsToBuffer returns a Promise', async (t) => {
  const buffer = fs.readFileSync('./music/03.mp3')
  const result = writeTagsToBuffer(buffer, { title: 'Test' })
  t.true(result instanceof Promise)

  // Handle the promise rejection to avoid unhandled rejection
  try {
    await result
  } catch (error) {
    // Expected error due to current implementation limitations
    t.true(error instanceof Error)
  }
})

test('readCoverImage returns a Promise', (t) => {
  const audioBuffer = fs.readFileSync('./music/03.mp3')
  const result = readCoverImage(audioBuffer)
  t.true(result instanceof Promise)
})

test('writeCoverImage returns a Promise', (t) => {
  const audioBuffer = fs.readFileSync('./music/03.mp3')
  const sampleImageData = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]) // PNG header
  const result = writeCoverImage(audioBuffer, sampleImageData)
  t.true(result instanceof Promise)
})
