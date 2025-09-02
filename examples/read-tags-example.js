const { readTags } = require('../index.js')

/**
 * Example: Read audio file tags
 * Usage: node examples/read-tags-example.js <file-path>
 *
 * This example reads the primary metadata tag from an audio file
 * and displays it as formatted JSON.
 */

async function main() {
  // Get file path from command line arguments
  const filePath = process.argv[2]

  if (!filePath) {
    console.error('Usage: node examples/read-tags-example.js <file-path>')
    console.error('Example: node examples/read-tags-example.js ./music/01.mp3')
    process.exit(1)
  }

  try {
    console.log(`Reading tags from: ${filePath}`)

    // Read the primary tag from the audio file
    const tags = await readTags(filePath)

    if (tags.title || tags.artists || tags.album) {
      console.log('Found tags:')
      console.log(JSON.stringify(tags, null, 2))
    } else {
      console.log('No tags found in the audio file')
    }
  } catch (error) {
    console.error('Error:', error.message)
    process.exit(1)
  }
}

// Run if this file is executed directly
if (require.main === module) {
  main()
}

module.exports = { main }
