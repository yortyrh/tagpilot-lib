import { Bench } from 'tinybench'

async function runBenchmark() {
  const b = new Bench()
  await b.run()

  console.table(b.table())
}

runBenchmark().catch(console.error)
