const fs = require('fs')
const { readTagsFromBuffer, readTags } = require('../index.js')

/**
 * Example: Read audio file tags from buffer
 * Usage: node examples/read-tags-from-buffer-example.js <file-path>
 *
 * This example demonstrates how to:
 * 1. Read an audio file into a buffer
 * 2. Extract tags from the buffer using readTagsFromBuffer
 * 3. Compare with reading tags directly from file
 * 4. Show the difference between file-based and buffer-based reading
 */

async function main() {
  // Get file path from command line arguments
  const filePath = process.argv[2]

  if (!filePath) {
    console.error('Usage: node examples/read-tags-from-buffer-example.js <file-path>')
    console.error('Example: node examples/read-tags-from-buffer-example.js ./music/01.mp3')
    process.exit(1)
  }

  try {
    console.log(`=== Reading tags from buffer: ${filePath} ===\n`)

    // Step 1: Read the file into a buffer
    console.log('1. Reading file into buffer...')
    const buffer = fs.readFileSync(filePath)
    console.log(`   File size: ${buffer.length} bytes`)
    console.log(`   Buffer type: ${buffer.constructor.name}`)
    console.log()

    // Step 2: Read tags from buffer
    console.log('2. Reading tags from buffer...')
    const bufferTags = await readTagsFromBuffer(buffer)

    if (bufferTags.title || bufferTags.artists || bufferTags.album) {
      console.log('Tags found in buffer:')
      console.log(JSON.stringify(bufferTags, null, 2))
    } else {
      console.log('No tags found in buffer')
    }
    console.log()

    // Step 3: Read tags directly from file for comparison
    console.log('3. Reading tags directly from file for comparison...')
    const fileTags = await readTags(filePath)

    if (fileTags.title || fileTags.artists || fileTags.album) {
      console.log('Tags found in file:')
      console.log(JSON.stringify(fileTags, null, 2))
    } else {
      console.log('No tags found in file')
    }
    console.log()

    // Step 4: Compare results
    console.log('4. Comparing results...')
    const bufferHasTags = !!(bufferTags.title || bufferTags.artists || bufferTags.album)
    const fileHasTags = !!(fileTags.title || fileTags.artists || fileTags.album)

    if (bufferHasTags === fileHasTags) {
      console.log('✅ Buffer and file reading produce the same result')

      if (bufferHasTags) {
        // Check if the tags are identical
        const bufferStr = JSON.stringify(bufferTags)
        const fileStr = JSON.stringify(fileTags)

        if (bufferStr === fileStr) {
          console.log('✅ Tags are identical between buffer and file reading')
        } else {
          console.log('⚠️  Tags differ between buffer and file reading')
          console.log('   This might indicate a timing issue or file modification')
        }
      }
    } else {
      console.log('❌ Buffer and file reading produce different results')
      console.log(`   Buffer has tags: ${bufferHasTags}`)
      console.log(`   File has tags: ${fileHasTags}`)
    }

    console.log('\n=== Use Cases for Buffer Reading ===')
    console.log('• Processing audio data from network requests')
    console.log('• Working with audio data in memory')
    console.log('• Processing audio streams')
    console.log('• Avoiding file system I/O for better performance')
    console.log('• Working with audio data from databases or cloud storage')

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
