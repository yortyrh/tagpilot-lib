const { readTags, writeTags } = require('../index.js')

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
  const filePath = process.argv[2]

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

    if (originalTags.title || originalTags.artist || originalTags.album) {
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
      artist: 'Sample Artist',
      album: 'Sample Album',
      year: 2024,
      genre: 'Sample Genre',
      track: 1,
      trackTotal: 12,
      albumArtist: 'Sample Album Artist',
      comment: 'This is a sample comment for demonstration purposes',
      disc: 1,
      discTotal: 3,
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
    console.log(`   Artist: "${sampleTags.artist}"`)
    console.log(`   Album: "${sampleTags.album}"`)
    console.log(`   Year: ${sampleTags.year}`)
    console.log(`   Genre: "${sampleTags.genre}"`)
    console.log(`   Track: ${sampleTags.track}/${sampleTags.trackTotal}`)
    console.log(`   Album Artist: "${sampleTags.albumArtist}"`)
    console.log(`   Comment: "${sampleTags.comment}"`)
    console.log(`   Disc: ${sampleTags.disc}/${sampleTags.discTotal}`)

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
