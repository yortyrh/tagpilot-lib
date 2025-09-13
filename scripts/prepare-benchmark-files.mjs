#!/usr/bin/env node
/**
 * Prepare test files for benchmarking by:
 * 1. Converting music files from test-files to a temp directory
 * 2. Adding metadata and cover images to each file
 * 3. Using different image formats (png/gif/jpg) from dummyimage.com
 */

import fs from 'fs/promises'
import path from 'path'
import { writeTagsToBuffer } from '../index.js'
import { safeReadFileAsync, validateAndResolvePath, isValidFileName } from './security-utils.mjs'

// Image formats to alternate between
const IMAGE_FORMATS = ['png', 'gif', 'jpg']
// IMAGE_FORMAT to content type
const IMAGE_CONTENT_TYPES = {
  png: 'image/png',
  gif: 'image/gif',
  jpg: 'image/jpeg',
}
const IMAGE_SIZES = ['500x500', '1000x1000', '1600x1600', '2000x2000']
const IMAGES = new Map()

async function getImageBuffer(imageFormat, imageSize) {
  const key = `${imageFormat}-${imageSize}`
  if (IMAGES.has(key)) {
    return IMAGES.get(key)
  }
  try {
    const imageUrl = `https://dummyimage.com/${imageSize}/09f/fff.${imageFormat}&text=${encodeURIComponent(imageFormat.toUpperCase() + ' ' + imageSize)}`
    console.log(imageUrl)
    const imageBuffer = await fetch(imageUrl).then((res) => res.arrayBuffer())
    const buffer = Buffer.from(imageBuffer)
    IMAGES.set(key, buffer)
    return buffer
  } catch (error) {
    console.warn(`Failed to get image buffer for ${key}:`, error.message)
    return null
  }
}

// Sample metadata to add to files
const getSampleMetadata = async (index, fileName) => {
  // generate random format and size
  const imageFormat = IMAGE_FORMATS[Math.floor(Math.random() * IMAGE_FORMATS.length)]
  const imageSize = IMAGE_SIZES[Math.floor(Math.random() * IMAGE_SIZES.length)]
  const imageBuffer = await getImageBuffer(imageFormat, imageSize)
  return {
    title: `Sample Title ${fileName}`,
    artists: [`First Artist ${index}`, `Second Artist ${index}`],
    albumArtists: [`First Album Artist ${index}`, `Second Album Artist ${index}`],
    track: { no: index, of: index + 1 },
    disc: { no: index, of: index + 1 },
    comment: `Test Comment ${index}`,
    album: `Test Album ${index}`,
    year: 1990 + (index % 30),
    genre: `Test Genre ${index}`,
    image: {
      data: imageBuffer,
      mimeType: IMAGE_CONTENT_TYPES[imageFormat],
      description: `Test Image ${index} ${imageFormat} ${imageSize}`,
    },
  }
}

async function addMetadataToFile(filePath, metadata) {
  try {
    // Security: Validate file path before reading
    const safePath = validateAndResolvePath(filePath, process.cwd())
    if (!safePath) {
      console.warn(`Skipping unsafe file path: ${filePath}`)
      return false
    }

    // Add metadata
    const buffer = await safeReadFileAsync(safePath, process.cwd())
    if (!buffer) {
      console.warn(`Failed to read file: ${filePath}`)
      return false
    }

    const newBuffer = await writeTagsToBuffer(buffer, metadata)
    await fs.writeFile(safePath, newBuffer)
    return true
  } catch (error) {
    console.warn(`Failed to add metadata to ${filePath}:`, error.message)
    return false
  }
}

async function prepareTestFiles() {
  const dirName = 'benchmark-files'
  const dirPath = path.join(process.cwd(), dirName)

  // Get audio files from benchmark-files, prioritizing formats that support metadata well
  const audioFiles = []
  const entries = await fs.readdir(dirPath, { withFileTypes: true })

  // Process files in order of metadata support likelihood
  const supportedFormats = ['.mp3', '.flac', '.ogg', '.opus', '.aiff']

  for (const entry of entries) {
    if (entry.isFile()) {
      // Security: Validate file name before processing
      if (!isValidFileName(entry.name)) {
        console.warn(`Skipping potentially unsafe file: ${entry.name}`)
        continue
      }

      const ext = path.extname(entry.name).toLowerCase()
      if (supportedFormats.includes(ext)) {
        const fullPath = path.join(dirPath, entry.name)
        // Additional security check
        const safePath = validateAndResolvePath(entry.name, dirPath)
        if (safePath) {
          audioFiles.push(safePath)
        } else {
          console.warn(`Skipping unsafe file path: ${entry.name}`)
        }
      }
    }
  }

  console.log(`Found ${audioFiles.length} audio files, processing`)

  let processedCount = 0
  let successCount = 0

  for (let i = 0; i < audioFiles.length; i++) {
    const sourceFile = audioFiles[i]
    const fileName = path.basename(sourceFile)

    try {
      // Select metadata and image format
      const metadata = await getSampleMetadata(i, fileName)

      // Add metadata and cover image
      const success = await addMetadataToFile(sourceFile, metadata)

      if (success) {
        successCount++
        console.log(`✓ Processed ${fileName}`)
      } else {
        console.log(`✗ Failed to process ${fileName}`)
      }
    } catch (error) {
      console.error(`Error processing ${fileName}:`, error.message)
    }

    processedCount++
  }

  console.log(`\nPreparation complete:`)
  console.log(`- Processed: ${processedCount} files`)
  console.log(`- Success: ${successCount} files`)
  console.log(`- Temp directory: ${dirPath}`)

  return dirPath
}

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  prepareTestFiles().catch(console.error)
}

export { prepareTestFiles }
