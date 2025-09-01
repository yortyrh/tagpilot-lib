const fs = require('fs')
const { readCoverImage, writeCoverImage } = require('../index.js')

/**
 * Example: Cover image operations with buffers
 * Usage: node examples/cover-image-buffer-example.js <audio-file-path> <image-file-path>
 *
 * This example demonstrates how to:
 * 1. Read cover image from an audio buffer
 * 2. Write a cover image to an audio buffer
 * 3. Save the modified buffer
 */

async function main() {
  // Get file paths from command line arguments
  const audioFilePath = process.argv[2]
  const imageFilePath = process.argv[3]

  if (!audioFilePath || !imageFilePath) {
    console.error('Usage: node examples/cover-image-buffer-example.js <audio-file-path> <image-file-path>')
    console.error('Example: node examples/cover-image-buffer-example.js ./music/03.mp3 ./cover.jpg')
    process.exit(1)
  }

  try {
    console.log(`=== Cover Image Buffer Operations ===`)
    console.log(`Audio file: ${audioFilePath}`)
    console.log(`Image file: ${imageFilePath}\n`)

    // Check if files exist
    if (!fs.existsSync(audioFilePath)) {
      console.error(`❌ Audio file not found: ${audioFilePath}`)
      process.exit(1)
    }

    if (!fs.existsSync(imageFilePath)) {
      console.error(`❌ Image file not found: ${imageFilePath}`)
      process.exit(1)
    }

    // Read the audio file into buffer
    console.log('Reading audio file...')
    const audioBuffer = fs.readFileSync(audioFilePath)
    console.log(`   ✅ Audio loaded: ${audioBuffer.length} bytes`)

    // Read the image file into buffer
    console.log('Reading image file...')
    const imageBuffer = fs.readFileSync(imageFilePath)
    console.log(`   ✅ Image loaded: ${imageBuffer.length} bytes`)

    // Read existing cover image from buffer
    console.log('Reading existing cover image from buffer...')
    try {
      const existingCover = await readCoverImage(audioBuffer)
      if (existingCover) {
        console.log(`   ✅ Existing cover image found: ${existingCover.length} bytes`)
      } else {
        console.log('   ℹ️  No existing cover image found')
      }
    } catch (error) {
      console.log(`   ⚠️  Error reading cover image: ${error.message}`)
    }
    console.log()

    // Write cover image to buffer
    console.log('Writing cover image to buffer...')
    const modifiedAudioBuffer = await writeCoverImage(audioBuffer, imageBuffer)
    console.log(`   ✅ Cover image written to buffer`)
    console.log(`   Modified audio size: ${modifiedAudioBuffer.length} bytes`)

    // Read cover image from modified buffer to verify
    console.log('Verifying cover image was written...')
    try {
      const newCover = await readCoverImage(modifiedAudioBuffer)
      if (newCover) {
        console.log(`   ✅ Cover image found in modified buffer: ${newCover.length} bytes`)
      } else {
        console.log('   ⚠️  No cover image found in modified buffer')
      }
    } catch (error) {
      console.log(`   ⚠️  Error reading cover image: ${error.message}`)
    }
    console.log()

    // Save the modified buffer to a new file
    const outputPath = audioFilePath.replace(/\.[^/.]+$/, '-with-cover$&')
    console.log('Saving modified buffer to file...')
    fs.writeFileSync(outputPath, modifiedAudioBuffer)
    console.log(`   ✅ File saved: ${outputPath}`)

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
