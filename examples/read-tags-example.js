const { readTags } = require('../index.js')

/**
 * Example: Read audio file tags
 * Usage: node examples/read-tags-example.js <file-path>
 *
 * This example reads all available metadata tags from an audio file
 * and displays them as formatted JSON.
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

    // Read all tags from the audio file
    const allTags = await readTags(filePath)

    if (allTags.length === 0) {
      console.log('No tags found in the audio file')
    } else if (allTags.length === 1) {
      console.log('Found 1 tag:')
      console.log(JSON.stringify(allTags[0], null, 2))
    } else {
      console.log(`Found ${allTags.length} tags:`)
      allTags.forEach((tag, index) => {
        console.log(`\n--- Tag ${index + 1} ---`)
        console.log(JSON.stringify(tag, null, 2))
      })
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
