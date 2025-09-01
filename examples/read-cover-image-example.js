const fs = require('fs')
const { readCoverImage } = require('../index.js')

/**
 * Example: Read cover image and return as data URL
 * Usage: node examples/read-cover-image-example.js <audio-file-path>
 *
 * This example demonstrates how to:
 * 1. Read cover image from an audio buffer
 * 2. Convert the cover image to a data URL
 * 3. Display or save the data URL
 */

function bufferToDataURL(buffer, mimeType = 'image/jpeg') {
  const base64 = buffer.toString('base64')
  return `data:${mimeType};base64,${base64}`
}

function detectMimeType(buffer) {
  // Simple MIME type detection based on file signatures
  const signature = buffer.slice(0, 4)

  if (signature[0] === 0xff && signature[1] === 0xd8 && signature[2] === 0xff) {
    return 'image/jpeg'
  }
  if (signature[0] === 0x89 && signature[1] === 0x50 && signature[2] === 0x4e && signature[3] === 0x47) {
    return 'image/png'
  }
  if (signature[0] === 0x47 && signature[1] === 0x49 && signature[2] === 0x46) {
    return 'image/gif'
  }
  if (signature[0] === 0x42 && signature[1] === 0x4d) {
    return 'image/bmp'
  }
  if (signature[0] === 0x49 && signature[1] === 0x49 && signature[2] === 0x2a && signature[3] === 0x00) {
    return 'image/tiff'
  }
  if (signature[0] === 0x4d && signature[1] === 0x4d && signature[2] === 0x00 && signature[3] === 0x2a) {
    return 'image/tiff'
  }

  // Default to JPEG if unknown
  return 'image/jpeg'
}

async function main() {
  // Get file path from command line arguments
  const audioFilePath = process.argv[2]

  if (!audioFilePath) {
    console.error('Usage: node examples/read-cover-image-example.js <audio-file-path>')
    console.error('Example: node examples/read-cover-image-example.js ./music/03.mp3')
    process.exit(1)
  }

  try {
    console.log(`=== Reading Cover Image as Data URL ===`)
    console.log(`Audio file: ${audioFilePath}\n`)

    // Check if file exists
    if (!fs.existsSync(audioFilePath)) {
      console.error(`❌ Audio file not found: ${audioFilePath}`)
      process.exit(1)
    }

    // Read the audio file into buffer
    console.log('Reading audio file...')
    const audioBuffer = fs.readFileSync(audioFilePath)
    console.log(`   ✅ Audio loaded: ${audioBuffer.length} bytes`)

    // Read cover image from buffer
    console.log('Reading cover image from buffer...')
    const coverImageBuffer = await readCoverImage(audioBuffer)

    if (coverImageBuffer) {
      console.log(`   ✅ Cover image found: ${coverImageBuffer.length} bytes`)

      // Detect MIME type
      const mimeType = detectMimeType(coverImageBuffer)
      console.log(`   📋 Detected MIME type: ${mimeType}`)

      // Convert to data URL
      console.log('Converting to data URL...')
      const dataURL = bufferToDataURL(coverImageBuffer, mimeType)
      console.log(`   ✅ Data URL generated (${dataURL.length} characters)`)

      // Display data URL (truncated for readability)
      console.log('\n=== Cover Image Data URL ===')
      console.log(dataURL.substring(0, 100) + '...')
      console.log('(truncated for display)')

      // Save data URL to file
      const outputPath = audioFilePath.replace(/\.[^/.]+$/, '-cover-dataurl.txt')
      console.log('\nSaving data URL to file...')
      fs.writeFileSync(outputPath, dataURL)
      console.log(`   ✅ Data URL saved: ${outputPath}`)

      // Save cover image as separate file
      const imageOutputPath = audioFilePath.replace(/\.[^/.]+$/, '-cover.' + mimeType.split('/')[1])
      console.log('Saving cover image as separate file...')
      fs.writeFileSync(imageOutputPath, coverImageBuffer)
      console.log(`   ✅ Cover image saved: ${imageOutputPath}`)

      console.log('\n=== Use Cases for Data URL ===')
      console.log('• Embedding in HTML: <img src="data:image/jpeg;base64,...">')
      console.log('• CSS backgrounds: background-image: url("data:image/jpeg;base64,...")')
      console.log('• Web applications: Display cover art without separate files')
      console.log('• Email attachments: Embed images directly in HTML emails')
      console.log('• API responses: Return cover art as part of JSON payload')
    } else {
      console.log('   ℹ️  No cover image found in audio file')

      console.log('\n=== No Cover Image Found ===')
      console.log('This audio file does not contain any cover art.')
      console.log('You can add cover art using the writeCoverImage function.')
    }

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

module.exports = { main, bufferToDataURL, detectMimeType }
