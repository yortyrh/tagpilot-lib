import { Bench } from 'tinybench'
import { readTags } from '../index.js'

async function readTagsSync(filePath: string) {
  // Simulate synchronous operation for comparison
  return await readTags(filePath)
}

async function runBenchmark() {
  const b = new Bench()

  b.add('Native readTags', async () => {
    await readTags('./music/01.mp3')
  })

  b.add('JavaScript wrapper', async () => {
    await readTagsSync('./music/01.mp3')
  })

  await b.run()

  console.table(b.table())
}

runBenchmark().catch(console.error)
