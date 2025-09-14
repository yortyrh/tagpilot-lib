const { readTags, clearTags } = require('../index.js')

/**
 * Example: Clear audio file tags
 * Usage: node examples/clear-tags-example.js <file-path>
 *
 * This example demonstrates how to clear all metadata tags from an audio file.
 * It first reads the existing tags, then clears them, and finally reads the file
 * again to confirm the tags have been cleared.
 */

async function main() {
  // Get file path from command line arguments
  const filePath = process.argv[2]
  // validate the Path Traversal vulnerability
  // convert the path to a relative path
  const filePathRelative = path.relative(process.cwd(), filePath)
  if (filePathRelative.includes('..')) {
    console.error('❌ Path Traversal vulnerability detected')
    process.exit(1)
  }

  if (!filePath) {
    console.error('Usage: node examples/clear-tags-example.js <file-path>')
    console.error('Example: node examples/clear-tags-example.js ./music/01.mp3')
    process.exit(1)
  }

  try {
    console.log(`=== Clearing tags from: ${filePath} ===\n`)

    // Step 1: Read the original tags
    console.log('1. Reading original tags...')
    const originalTags = await readTags(filePath)

    if (originalTags.title || originalTags.artists || originalTags.album) {
      console.log('Original tags found:')
      console.log(JSON.stringify(originalTags, null, 2))
    } else {
      console.log('No tags found in the audio file')
    }
    console.log()

    // Step 2: Clear all tags
    console.log('2. Clearing all tags...')
    await clearTags(filePath)
    console.log('Tags cleared successfully!')
    console.log()

    // Step 3: Verify tags have been cleared
    console.log('3. Verifying tags have been cleared...')
    const clearedTags = await readTags(filePath)

    if (clearedTags.title || clearedTags.artists || clearedTags.album) {
      console.log('Warning: Some tags still remain:')
      console.log(JSON.stringify(clearedTags, null, 2))
    } else {
      console.log('✓ All tags have been successfully cleared!')
      console.log('File now contains no metadata tags.')
    }

    console.log('\n=== Operation completed ===')
  } catch (error) {
    console.error('Error:', error.message)
    process.exit(1)
  }
}

// Run if this file is executed directly
if (require.main === module) {
  main()
}
