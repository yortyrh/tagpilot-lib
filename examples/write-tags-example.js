const { readTags, writeTags } = require('../index.js')
const { validatePath } = require('./helper.js')

/**
 * Example: Write sample audio file tags
 * Usage: node examples/write-tags-example.js <file-path>
 *
 * This example demonstrates how to:
 * 1. Read existing tags from an audio file (if any)
 * 2. Write sample tag data to the file
 * 3. Verify the changes by reading the file again
 */

async function main() {
  // Get file path from command line arguments
  const filePath = validatePath(process.argv[2], process.cwd())

  if (!filePath) {
    console.error('Usage: node examples/write-tags-example.js <file-path>')
    console.error('Example: node examples/write-tags-example.js ./music/01.mp3')
    process.exit(1)
  }

  try {
    console.log(`=== Writing sample tags to: ${filePath} ===\n`)

    // Step 1: Read the original tags (if any)
    console.log('1. Reading original tags...')
    const originalTags = await readTags(filePath)

    if (originalTags.title || originalTags.artists || originalTags.album) {
      console.log('Original tags found:')
      console.log(JSON.stringify(originalTags, null, 2))
    } else {
      console.log('No tags found in the file')
    }
    console.log()

    // Step 2: Create sample tags
    console.log('2. Creating sample tags...')
    const sampleTags = {
      title: 'Sample Song Title',
      artists: ['Sample Artist'],
      album: 'Sample Album',
      year: 2024,
      genre: 'Sample Genre',
      track: { no: 1, of: 12 },
      albumArtists: ['Sample Album Artist'],
      comment: 'This is a sample comment for demonstration purposes',
      disc: { no: 1, of: 3 },
    }

    console.log('Sample tags to write:')
    console.log(JSON.stringify(sampleTags, null, 2))
    console.log()

    // Step 3: Write the sample tags to the file
    console.log('3. Writing sample tags to file...')
    await writeTags(filePath, sampleTags)
    console.log('✅ Sample tags written successfully!')
    console.log()

    // Step 4: Verify the changes by reading the file again
    console.log('4. Verifying changes...')
    const updatedTags = await readTags(filePath)

    console.log('Updated tags:')
    console.log(JSON.stringify(updatedTags, null, 2))
    console.log()

    // Step 5: Show what was written
    console.log('5. Summary of sample data written:')
    console.log(`   Title: "${sampleTags.title}"`)
    console.log(`   Artists: "${sampleTags.artists?.join(', ')}"`)
    console.log(`   Album: "${sampleTags.album}"`)
    console.log(`   Year: ${sampleTags.year}`)
    console.log(`   Genre: "${sampleTags.genre}"`)
    console.log(`   Track: ${sampleTags.track?.no || 'N/A'} of ${sampleTags.track?.of || 'N/A'}`)
    console.log(`   Album Artists: "${sampleTags.albumArtists?.join(', ')}"`)
    console.log(`   Comment: "${sampleTags.comment}"`)
    console.log(`   Disc: ${sampleTags.disc?.no || 'N/A'} of ${sampleTags.disc?.of || 'N/A'}`)

    console.log('\n✅ Sample tags written successfully!')
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
