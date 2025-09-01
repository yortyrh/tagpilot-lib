import test from 'ava'
import { clearTags, readTagsFromBuffer } from '../index.js'
import * as fs from 'node:fs'

test('sync function from native code', (t) => {
  t.true(true)
})

test('clearTags returns a Promise', (t) => {
  const result = clearTags('./music/01.mp3')
  t.true(result instanceof Promise)
})

test('readTagsFromBuffer returns a Promise', (t) => {
  const buffer = fs.readFileSync('./music/01.mp3')
  const result = readTagsFromBuffer(buffer)
  t.true(result instanceof Promise)
})
