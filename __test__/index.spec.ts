import test from 'ava'
import { clearTags } from '../index.js'

test('sync function from native code', (t) => {
  t.true(true)
})

test('clearTags returns a Promise', (t) => {
  const result = clearTags('./music/01.mp3')
  t.true(result instanceof Promise)
})
