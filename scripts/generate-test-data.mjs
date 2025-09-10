#!/usr/bin/env node

import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

// MIME type mapping for audio files
const mimeTypes = {
  '.mp3': 'audio/mpeg',
  '.aac': 'audio/aac',
  '.flac': 'audio/flac',
  '.m4a': 'audio/mp4',
  '.ogg': 'audio/ogg',
  '.opus': 'audio/opus',
  '.wav': 'audio/wav',
  '.aiff': 'audio/aiff',
  '.wv': 'audio/wavpack',
  '.spx': 'audio/ogg',
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.png': 'image/png',
  '.gif': 'image/gif',
  '.json': 'application/json',
}

function getMimeType(fileName) {
  const ext = path.extname(fileName).toLowerCase()
  return mimeTypes[ext] || 'application/octet-stream'
}

function generateTestData() {
  const testFilesDir = path.join(__dirname, '..', 'test-files')
  const outputFile = path.join(__dirname, '..', '__test__', 'data', 'test-data.ts')

  console.log('Reading files from:', testFilesDir)

  // Read all files from test-files directory
  const files = fs.readdirSync(testFilesDir)
  const testData = []

  for (const fileName of files) {
    const filePath = path.join(testFilesDir, fileName)
    const stats = fs.statSync(filePath)

    // Skip directories
    if (stats.isDirectory()) {
      continue
    }

    console.log(`Processing: ${fileName}`)

    try {
      // Read file as buffer
      const data = fs.readFileSync(filePath)
      const mimeType = getMimeType(fileName)

      testData.push({
        data: data,
        mimeType: mimeType,
        fileName: fileName,
      })
    } catch (error) {
      console.error(`Error reading file ${fileName}:`, error.message)
    }
  }

  // Generate TypeScript file content
  const tsContent = `// Auto-generated test data file
// Generated on: ${new Date().toISOString()}
// Total files: ${testData.length}

export interface TestFile {
  data: Buffer;
  mimeType: string;
  fileName: string;
}

export const testFiles: TestFile[] = [
${testData
  .map((file, index) => {
    // Convert buffer to base64 for embedding in TypeScript
    const base64Data = file.data.toString('base64')
    return `  {
    data: Buffer.from('${base64Data}', 'base64'),
    mimeType: '${file.mimeType}',
    fileName: '${file.fileName}'
  }${index < testData.length - 1 ? ',' : ''}`
  })
  .join('\n')}
];

// Helper function to get files by extension
export function getFilesByExtension(extension: string): TestFile[] {
  return testFiles.filter(file => file.fileName.toLowerCase().endsWith(extension.toLowerCase()));
}

// Helper function to get files by MIME type
export function getFilesByMimeType(mimeType: string): TestFile[] {
  return testFiles.filter(file => file.mimeType === mimeType);
}

// Helper function to get a specific file by name
export function getFileByName(fileName: string): TestFile | undefined {
  return testFiles.find(file => file.fileName === fileName);
}

// Export individual file arrays for convenience
export const mp3Files = getFilesByExtension('.mp3');
export const flacFiles = getFilesByExtension('.flac');
export const m4aFiles = getFilesByExtension('.m4a');
export const aacFiles = getFilesByExtension('.aac');
export const oggFiles = getFilesByExtension('.ogg');
export const wavFiles = getFilesByExtension('.wav');
export const aiffFiles = getFilesByExtension('.aiff');
export const opusFiles = getFilesByExtension('.opus');
export const wvFiles = getFilesByExtension('.wv');
export const spxFiles = getFilesByExtension('.spx');
export const imageFiles = testFiles.filter(file => file.mimeType.startsWith('image/'));
export const audioFiles = testFiles.filter(file => file.mimeType.startsWith('audio/'));
`

  // Ensure __test__ directory exists
  const testDir = path.dirname(outputFile)
  if (!fs.existsSync(testDir)) {
    fs.mkdirSync(testDir, { recursive: true })
  }

  // Write the TypeScript file
  fs.writeFileSync(outputFile, tsContent, 'utf8')

  console.log(`\n✅ Generated test data file: ${outputFile}`)
  console.log(`📊 Total files processed: ${testData.length}`)
  console.log(`📁 File types found:`)

  // Count file types
  const typeCounts = {}
  testData.forEach((file) => {
    const ext = path.extname(file.fileName).toLowerCase()
    typeCounts[ext] = (typeCounts[ext] || 0) + 1
  })

  Object.entries(typeCounts)
    .sort(([, a], [, b]) => b - a)
    .forEach(([ext, count]) => {
      console.log(`   ${ext}: ${count} files`)
    })
}

// Run the script
if (import.meta.url === `file://${process.argv[1]}`) {
  try {
    generateTestData()
  } catch (error) {
    console.error('Error generating test data:', error)
    process.exit(1)
  }
}

export { generateTestData }
