const { readTags, writeTags } = require('../index.js')

/**
 * Example: Write audio file tags
 * Usage: node examples/write-tags-example.js <file-path>
 *
 * This example demonstrates how to:
 * 1. Read existing tags from an audio file
 * 2. Modify some tag values
 * 3. Write the updated tags back to the file
 * 4. Verify the changes by reading the file again
 */

async function main() {
  // Get file path from command line arguments
  const filePath = process.argv[2]

  if (!filePath) {
    console.error('Usage: node examples/write-tags-example.js <file-path>')
    console.error('Example: node examples/write-tags-example.js ./music/01.mp3')
    process.exit(1)
  }

  try {
    console.log(`=== Writing tags to: ${filePath} ===\n`)

    // Step 1: Read the original tags
    console.log('1. Reading original tags...')
    const originalTags = await readTags(filePath)
    console.log('Original tags:')
    console.log(JSON.stringify(originalTags, null, 2))
    console.log()

    // Step 2: Create modified tags
    console.log('2. Creating modified tags...')
    const modifiedTags = {
      ...originalTags,
      title: originalTags.title ? `[MODIFIED] ${originalTags.title}` : 'Modified Title',
      comment: originalTags.comment
        ? `${originalTags.comment} [Modified by writeTags example]`
        : 'Modified by writeTags example',
      year: (originalTags.year || 2000) + 1, // Increment year by 1
      genre: originalTags.genre || 'Modified Genre',
    }

    console.log('Modified tags:')
    console.log(JSON.stringify(modifiedTags, null, 2))
    console.log()

    // Step 3: Write the modified tags back to the file
    console.log('3. Writing modified tags to file...')
    await writeTags(filePath, modifiedTags)
    console.log('✅ Tags written successfully!')
    console.log()

    // Step 4: Verify the changes by reading the file again
    console.log('4. Verifying changes...')
    const updatedTags = await readTags(filePath)
    console.log('Updated tags:')
    console.log(JSON.stringify(updatedTags, null, 2))
    console.log()

    // Step 5: Show what changed
    console.log('5. Summary of changes:')
    if (originalTags.title !== updatedTags.title) {
      console.log(`   Title: "${originalTags.title}" → "${updatedTags.title}"`)
    }
    if (originalTags.comment !== updatedTags.comment) {
      console.log(`   Comment: "${originalTags.comment}" → "${updatedTags.comment}"`)
    }
    if (originalTags.year !== updatedTags.year) {
      console.log(`   Year: ${originalTags.year} → ${updatedTags.year}`)
    }
    if (originalTags.genre !== updatedTags.genre) {
      console.log(`   Genre: "${originalTags.genre}" → "${updatedTags.genre}"`)
    }

    console.log('\n✅ Tag modification completed successfully!')
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
