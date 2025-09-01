import { Bench } from 'tinybench'
import { readTags } from '../index.js'

async function readTagsSync(filePath: string) {
  // Simulate synchronous operation for comparison
  return await readTags(filePath)
}

async function runBenchmark() {
  const b = new Bench()
  await b.run()

  console.table(b.table())
}

runBenchmark().catch(console.error)
