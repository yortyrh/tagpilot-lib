const { readTags, writeTags } = require('../index.js')
const { validatePath } = require('./helper.js')

/**
 * Example: Write sample audio file tags with images
 * Usage: node examples/write-tags-example.js <file-path> <front-cover-path> <back-cover-path>
 *
 * This example demonstrates how to:
 * 1. Read existing tags from an audio file (if any)
 * 2. Write sample tag data including cover and back images to the file
 * 3. Verify the changes by reading the file again
 *
 * The example includes:
 * - Basic metadata (title, artist, album, etc.)
 * - Front cover image (from provided file)
 * - Back cover image (from provided file)
 *
 * Supported image formats:
 * - JPEG (.jpg, .jpeg)
 * - PNG (.png)
 */

const fs = require('fs')
const path = require('path')

// Helper function to read image file and determine its MIME type
function readImageFile(imagePath) {
  const data = fs.readFileSync(imagePath)
  const ext = path.extname(imagePath).toLowerCase()
  let mimeType
  switch (ext) {
    case '.jpg':
    case '.jpeg':
      mimeType = 'image/jpeg'
      break
    case '.png':
      mimeType = 'image/png'
      break
    default:
      throw new Error(`Unsupported image format: ${ext}. Only JPEG and PNG are supported.`)
  }
  return { data: Buffer.from(data), mimeType }
}

async function main() {
  // Get file paths from command line arguments
  const filePath = validatePath(process.argv[2], process.cwd())
  const frontCoverPath = validatePath(process.argv[3], process.cwd())
  const backCoverPath = validatePath(process.argv[4], process.cwd())

  if (!filePath || !frontCoverPath || !backCoverPath) {
    console.error('Usage: node examples/write-tags-example.js <file-path> <front-cover-path> <back-cover-path>')
    console.error('Example: node examples/write-tags-example.js ./music/01.mp3 ./images/front.jpg ./images/back.png')
    process.exit(1)
  }

  // Validate image files
  try {
    fs.accessSync(frontCoverPath, fs.constants.R_OK)
    fs.accessSync(backCoverPath, fs.constants.R_OK)
  } catch (error) {
    console.error('Error: Cannot access image files.')
    console.error(`Front cover: ${frontCoverPath}`)
    console.error(`Back cover: ${backCoverPath}`)
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

    // Step 2: Create sample tags with images
    console.log('2. Creating sample tags with images...')

    // Read the cover and back images
    console.log(`   Reading front cover from: ${path.basename(frontCoverPath)}`)
    const coverImage = readImageFile(frontCoverPath)
    console.log(`   Reading back cover from: ${path.basename(backCoverPath)}`)
    const backImage = readImageFile(backCoverPath)

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
      // Add cover image as the primary image
      image: {
        data: coverImage.data,
        picType: 'CoverFront',
        mimeType: coverImage.mimeType,
        description: 'Front cover image',
      },
      // Add both front and back cover images
      allImages: [
        {
          data: coverImage.data,
          picType: 'CoverFront',
          mimeType: coverImage.mimeType,
          description: 'Front cover image',
        },
        {
          data: backImage.data,
          picType: 'CoverBack',
          mimeType: backImage.mimeType,
          description: 'Back cover image',
        },
      ],
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
    console.log('   Images:')
    console.log(`     - Cover Image: ${sampleTags.image?.data.length} bytes (${sampleTags.image?.mimeType})`)
    sampleTags.allImages?.forEach((img, index) => {
      console.log(
        `     - Image ${index + 1}: ${img.data.length} bytes (${img.mimeType}, ${img.picType}, "${img.description}")`,
      )
    })

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
