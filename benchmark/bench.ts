import { Bench } from 'tinybench'
import fs from 'fs/promises'
import path from 'path'
import { ChartJSNodeCanvas } from 'chartjs-node-canvas'
import { readTags } from '../index.js'
import { parseFile } from 'music-metadata'

// Test data setup
const BENCHMARK_FILES_DIR = path.join(process.cwd(), 'benchmark-files')
const SUPPORTED_FORMATS = ['.mp3', '.flac', '.ogg', '.opus', '.aiff']
let testFiles: string[] = []

async function generateBarChartImage(results: any[]) {
  // Filter and sort results by throughput
  const chartData = results
    .filter((r) => r && r['Task name'] && r['Throughput avg (ops/s)'])
    .map((r) => {
      const throughputStr = r['Throughput avg (ops/s)']
      const throughput =
        typeof throughputStr === 'string' ? Number(throughputStr.replace(/[^\d.]/g, '')) : Number(throughputStr) || 0

      return {
        name: r['Task name'],
        throughput: throughput,
      }
    })
    .sort((a, b) => b.throughput - a.throughput)

  if (chartData.length === 0) {
    console.log('No data available for chart image')
    return
  }

  // Create Chart.js configuration
  const chartJSNodeCanvas = new ChartJSNodeCanvas({
    width: 800,
    height: 400,
    backgroundColour: 'white',
  })

  const configuration = {
    type: 'bar' as const,
    data: {
      labels: chartData.map((d) => (d.name.includes('tagpilot-lib') ? 'tagpilot-lib' : 'music-metadata')),
      datasets: [
        {
          label: 'Throughput (ops/s)',
          data: chartData.map((d) => d.throughput),
          backgroundColor: chartData.map((d) => (d.name.includes('tagpilot-lib') ? '#4CAF50' : '#2196F3')),
          borderColor: chartData.map((d) => (d.name.includes('tagpilot-lib') ? '#388E3C' : '#1976D2')),
          borderWidth: 2,
        },
      ],
    },
    options: {
      responsive: true,
      plugins: {
        title: {
          display: true,
          text: 'Performance Comparison: tagpilot-lib vs music-metadata',
          font: {
            size: 20,
            weight: 'bold' as const,
          },
          color: '#333333',
        },
        subtitle: {
          display: true,
          text: 'Throughput (operations per second)',
          font: {
            size: 14,
          },
          color: '#666666',
        },
        legend: {
          display: true,
          position: 'top' as const,
          labels: {
            usePointStyle: true,
            font: {
              size: 12,
            },
          },
        },
        tooltip: {
          callbacks: {
            label: function (context: any) {
              const data = chartData[context.dataIndex]
              const percentage = ((data.throughput / Math.max(...chartData.map((d) => d.throughput))) * 100).toFixed(1)
              return `${data.throughput.toFixed(1)} ops/s (${percentage}%)`
            },
          },
        },
      },
      scales: {
        x: {
          title: {
            display: true,
            text: 'Library',
            font: {
              size: 14,
              weight: 'bold' as const,
            },
          },
          ticks: {
            font: {
              size: 12,
            },
          },
        },
        y: {
          title: {
            display: true,
            text: 'Operations per Second',
            font: {
              size: 14,
              weight: 'bold' as const,
            },
          },
          ticks: {
            font: {
              size: 12,
            },
            callback: function (value: any) {
              return value + ' ops/s'
            },
          },
          beginAtZero: true,
        },
      },
    },
  }

  try {
    const imageBuffer = await chartJSNodeCanvas.renderToBuffer(configuration)
    const imagePath = path.join(process.cwd(), 'benchmark-results.jpg')
    await fs.writeFile(imagePath, imageBuffer)
    console.log(`\n📊 Benchmark chart saved to: ${imagePath}`)
  } catch (error) {
    console.error('Error generating chart image:', (error as Error).message)
  }
}

function generateBarChart(results: any[]) {
  console.log('\n=== PERFORMANCE BAR CHART (Throughput - ops/s) ===\n')

  // Filter and sort results by throughput
  const chartData = results
    .filter((r) => r && r['Task name'] && r['Throughput avg (ops/s)'])
    .map((r) => {
      const throughputStr = r['Throughput avg (ops/s)']
      const throughput =
        typeof throughputStr === 'string' ? Number(throughputStr.replace(/[^\d.]/g, '')) : Number(throughputStr) || 0

      return {
        name: r['Task name'],
        throughput: throughput,
      }
    })
    .sort((a, b) => b.throughput - a.throughput)

  if (chartData.length === 0) {
    console.log('No data available for chart')
    return
  }

  const maxThroughput = Math.max(...chartData.map((d) => d.throughput))
  const barWidth = 50
  const scale = barWidth / maxThroughput

  chartData.forEach((data, index) => {
    const barLength = Math.round(data.throughput * scale)
    const bar = '█'.repeat(barLength)
    const padding = ' '.repeat(Math.max(0, barWidth - barLength))
    const percentage = ((data.throughput / maxThroughput) * 100).toFixed(1)

    console.log(
      `${(index + 1).toString().padStart(2)}. ${data.name.padEnd(35)} │${bar}${padding}│ ${data.throughput.toFixed(1).padStart(6)} ops/s (${percentage}%)`,
    )
  })

  console.log(`\n${' '.repeat(37)}└${'─'.repeat(barWidth)}┘`)
  console.log(`${' '.repeat(37)}0${' '.repeat(barWidth - 2)}${maxThroughput.toFixed(0)} ops/s`)
}

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

  console.log('Running benchmarks...')
  console.log('This may take a few minutes...\n')

  await bench.run()

  console.log('\n=== BENCHMARK RESULTS ===\n')
  console.table(bench.table())

  // Generate bar chart
  generateBarChart(bench.table())

  // Generate JPG image
  await generateBarChartImage(bench.table())

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
