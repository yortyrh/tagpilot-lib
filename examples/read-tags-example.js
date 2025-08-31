const { readTags } = require('../index.js')

/**
 * Simple example: read audio file tags and print as JSON
 * Usage: node examples/read-tags-example.js <file-path>
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

    // Read tags from the audio file (now returns object directly)
    const tags = await readTags(filePath)

    // Print the object as formatted JSON
    console.log(JSON.stringify(tags, null, 2))
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
