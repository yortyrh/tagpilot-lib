const fs = require('fs')
const path = require('path')
const { readTagsFromBuffer, writeTagsToBuffer, writeTags } = require('../index.js')
const { validatePath } = require('./helper.js')

// Security utility functions
function isValidFileName(fileName) {
  if (!fileName || typeof fileName !== 'string') return false
  const dangerousPatterns = [/\.\./, /\.\.\\/, /\.\.\//, /[<>:"|?*]/, /[\x00-\x1f]/, /^\./, /\/$/, /\\$/]
  return !dangerousPatterns.some((pattern) => pattern.test(fileName))
}

function validateFilePath(filePath, baseDir) {
  if (!filePath || typeof filePath !== 'string') return null
  try {
    const resolvedPath = path.resolve(baseDir, filePath)
    if (!resolvedPath.startsWith(path.resolve(baseDir))) return null
    return resolvedPath
  } catch {
    return null
  }
}

/**
 * Example: Write audio file tags to buffer
 * Usage: node examples/write-tags-to-buffer-example.js <file-path>
 *
 * This example demonstrates how to:
 * 1. Read an audio file into a buffer
 * 2. Read tags from the buffer
 * 3. Write new tags to the buffer
 * 4. Compare with file-based writing
 * 5. Show the difference between buffer and file approaches
 */

async function main() {
  // Get file path from command line arguments
  const filePath = validatePath(process.argv[2], process.cwd())

  if (!filePath) {
    console.error('Usage: node examples/write-tags-to-buffer-example.js <file-path>')
    console.error('Example: node examples/write-tags-to-buffer-example.js ./music/01.mp3')
    process.exit(1)
  }

  try {
    console.log(`=== Writing tags to buffer: ${filePath} ===\n`)

    // Security: Validate file path
    const fileName = path.basename(filePath)
    if (!isValidFileName(fileName)) {
      console.error(`❌ Unsafe file name detected: ${fileName}`)
      process.exit(1)
    }

    const safeFilePath = validateFilePath(filePath, process.cwd())
    if (!safeFilePath) {
      console.error(`❌ Unsafe file path detected: ${filePath}`)
      process.exit(1)
    }

    // Step 1: Read the file into a buffer
    console.log('1. Reading file into buffer...')
    const buffer = fs.readFileSync(safeFilePath)
    console.log(`   File size: ${buffer.length} bytes`)
    console.log(`   Buffer type: ${buffer.constructor.name}`)
    console.log()

    // Step 2: Read original tags from buffer
    console.log('2. Reading original tags from buffer...')
    const originalTags = await readTagsFromBuffer(buffer)

    if (originalTags.title || originalTags.artists || originalTags.album) {
      console.log('Original tags found:')
      console.log(JSON.stringify(originalTags, null, 2))
    } else {
      console.log('No tags found in buffer')
    }
    console.log()

    // Step 3: Create new tags to write
    console.log('3. Creating new tags to write...')
    const newTags = {
      title: 'Buffer Modified Title',
      artists: ['Buffer Modified Artist'],
      album: 'Buffer Modified Album',
      year: 2024,
      genre: 'Buffer Modified Genre',
      track: { no: 2, of: 15 },
      albumArtists: ['Buffer Modified Album Artist'],
      comment: 'This was modified using writeTagsToBuffer',
      disc: { no: 2, of: 2 },
    }

    console.log('New tags to write:')
    console.log(JSON.stringify(newTags, null, 2))
    console.log()

    // Step 4: Write tags to buffer
    console.log('4. Writing tags to buffer...')
    try {
      const modifiedBuffer = await writeTagsToBuffer(buffer, newTags)
      console.log(`✅ Tags written to buffer successfully!`)
      console.log(`   Original buffer size: ${buffer.length} bytes`)
      console.log(`   Modified buffer size: ${modifiedBuffer.length} bytes`)
      console.log()

      // Step 5: Read tags from modified buffer
      console.log('5. Reading tags from modified buffer...')
      const modifiedTags = await readTagsFromBuffer(modifiedBuffer)

      console.log('Tags from modified buffer:')
      console.log(JSON.stringify(modifiedTags, null, 2))
      console.log()

      // Step 6: Compare with file-based approach
      console.log('6. Comparing with file-based approach...')

      // Write to file for comparison
      await writeTags(filePath, newTags)
      const fileTags = await readTagsFromBuffer(fs.readFileSync(filePath))

      const bufferStr = JSON.stringify(modifiedTags)
      const fileStr = JSON.stringify(fileTags)

      if (bufferStr === fileStr) {
        console.log('✅ Buffer and file approaches produce identical results')
      } else {
        console.log('⚠️  Buffer and file approaches produce different results')
        console.log('   Buffer tags:', modifiedTags)
        console.log('   File tags:', fileTags)
      }
    } catch (error) {
      console.log(`⚠️  Buffer writing failed: ${error.message}`)
      console.log('   This might be due to format limitations or implementation issues')
      console.log('   The file-based approach is recommended for production use')
    }

    console.log('\n=== Use Cases for Buffer Writing ===')
    console.log('• Processing audio data in memory without file I/O')
    console.log('• Working with audio streams')
    console.log('• Processing audio data from network requests')
    console.log('• Avoiding file system operations for better performance')
    console.log('• Working with audio data in web applications')
    console.log('• Handling audio data from databases or cloud storage')

    console.log('\n=== Operation completed ===')
  } catch (error) {
    console.error('❌ Error:', error.message)
    process.exit(1)
  }
}

// Run if this file is executed directly
if (require.main === module) {
  main()
}

module.exports = { main }
