const fs = require('fs')
const { writeCoverImage } = require('../index.js')

/**
 * Example: Set cover image for audio file
 * Usage: node examples/cover-image-example.js <audio-file-path> <image-file-path> [output-file-path]
 *
 * This example demonstrates how to:
 * 1. Set a cover image to an audio file buffer
 * 2. Save the modified buffer to a file
 */

async function main() {
  // Get file paths from command line arguments
  const audioFilePath = process.argv[2]
  const imageFilePath = process.argv[3]
  const outputFilePath = process.argv[4] || audioFilePath

  if (!audioFilePath || !imageFilePath) {
    console.error('Usage: node examples/cover-image-example.js <audio-file-path> <image-file-path> [output-file-path]')
    console.error('Example: node examples/cover-image-example.js ./music/03.mp3 ./cover.jpg')
    console.error('Example: node examples/cover-image-example.js ./music/03.mp3 ./cover.jpg ./music/03-with-cover.mp3')
    process.exit(1)
  }

  try {
    console.log(`=== Setting Cover Image ===`)
    console.log(`Audio file: ${audioFilePath}`)
    console.log(`Image file: ${imageFilePath}`)
    console.log(`Output file: ${outputFilePath}\n`)

    // Check if files exist
    if (!fs.existsSync(audioFilePath)) {
      console.error(`❌ Audio file not found: ${audioFilePath}`)
      process.exit(1)
    }

    if (!fs.existsSync(imageFilePath)) {
      console.error(`❌ Image file not found: ${imageFilePath}`)
      process.exit(1)
    }

    // Read the audio file
    console.log('Reading audio file...')
    const audioData = fs.readFileSync(audioFilePath)
    console.log(`   ✅ Audio loaded: ${audioData.length} bytes`)

    // Read the image file
    console.log('Reading image file...')
    const imageData = fs.readFileSync(imageFilePath)
    console.log(`   ✅ Image loaded: ${imageData.length} bytes`)

    // Set the cover image
    console.log('Setting cover image...')
    const modifiedAudioData = await writeCoverImage(audioData, imageData)
    console.log(`   ✅ Cover image set successfully!`)
    console.log(`   Modified audio size: ${modifiedAudioData.length} bytes`)

    // Save the modified audio file
    console.log('Saving modified audio file...')
    fs.writeFileSync(outputFilePath, modifiedAudioData)
    console.log(`   ✅ File saved: ${outputFilePath}`)

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
