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
 * Image Handling:
 * - The 'image' field is used for the primary cover image (CoverFront)
 * - The 'allImages' field contains all images, including the cover
 * - When 'allImages' is provided, it takes precedence over 'image'
 * - The library ensures the cover image (CoverFront) is always first in 'allImages'
 *
 * In this example:
 * - We set both 'image' (for backward compatibility) and 'allImages'
 * - The front cover is added as both the primary image and first in allImages
 * - The back cover is added to allImages after the front cover
 * - The library will maintain this order (cover first, then others)
 *
 * Supported image formats:
 * - JPEG (.jpg, .jpeg)
 * - PNG (.png)
 *
 * Note: When reading tags:
 * - 'image' will contain the CoverFront image if present
 * - 'allImages' will contain all images in order (CoverFront first if present)
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

  // Check if all required arguments were provided
  if (!process.argv[2] || !process.argv[3] || !process.argv[4]) {
    console.error('Usage: node examples/write-tags-example.js <file-path> <front-cover-path> <back-cover-path>')
    console.error('Example: node examples/write-tags-example.js ./music/01.mp3 ./images/front.jpg ./images/back.png')
    process.exit(1)
  }

  // Check if any paths were invalid
  const invalidPaths = []
  if (!filePath) invalidPaths.push(['audio file', process.argv[2]])
  if (!frontCoverPath) invalidPaths.push(['front cover image', process.argv[3]])
  if (!backCoverPath) invalidPaths.push(['back cover image', process.argv[4]])

  if (invalidPaths.length > 0) {
    console.error('Error: Invalid path(s) provided:')
    invalidPaths.forEach(([type, path]) => {
      console.error(`  - ${type}: "${path}"`)
    })
    console.error('\nPaths must:')
    console.error('  - Be within the current directory')
    console.error('  - Not contain parent directory traversal (.., ./, etc.)')
    console.error('  - Not contain special characters (<>:"|?*)')
    console.error('  - Not be hidden files (starting with .)')
    process.exit(1)
  }

  // Validate image files
  try {
    fs.accessSync(frontCoverPath, fs.constants.R_OK)
    fs.accessSync(backCoverPath, fs.constants.R_OK)
  } catch (error) {
    console.error('Error: Cannot access image files.', error)
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
      // Set the primary cover image in 'image' field
      // This is for backward compatibility and convenience when only a cover is needed
      image: {
        data: coverImage.data,
        picType: 'CoverFront',
        mimeType: coverImage.mimeType,
        description: 'Front cover image',
      },
      // Set all images in 'allImages' field
      // This takes precedence over 'image' field
      // The library ensures CoverFront is always first in the list
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
    // Show primary cover image (from 'image' field)
    console.log('   Primary cover image:')
    console.log(`     ${sampleTags.image?.data.length} bytes (${sampleTags.image?.mimeType})`)

    // Show all images in order (from 'allImages' field)
    console.log('   All images (in order, cover first):')
    sampleTags.allImages?.forEach((img, index) => {
      console.log(`     ${index + 1}. ${img.picType}: ${img.data.length} bytes (${img.mimeType}, "${img.description}")`)
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
