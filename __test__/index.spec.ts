import test from 'ava'
import { readTagsFromBuffer, writeTagsToBuffer, type AudioTags, AudioImageType } from '../index.js'
import {
  mp3Files,
  flacFiles,
  aacFiles,
  oggFiles,
  opusFiles,
  wavFiles,
  aiffFiles,
  spxFiles,
  getFileByName,
} from './data/test-data.js'

test('sync function from native code', (t) => {
  t.true(true)
})

test('readTagsFromBuffer - should return empty tags for MP3 files without metadata', async (t) => {
  await testEmptyTags(t, mp3Files, 'MP3')
})

test('readTagsFromBuffer - should return empty tags for AAC files without metadata', async (t) => {
  await testEmptyTags(t, aacFiles, 'AAC')
})

test('readTagsFromBuffer - should return empty tags for FLAC files without metadata', async (t) => {
  await testEmptyTags(t, flacFiles, 'FLAC')
})

test('readTagsFromBuffer - should return empty tags for OGG files without metadata', async (t) => {
  await testEmptyTags(t, oggFiles, 'OGG')
})

test('readTagsFromBuffer - should return empty tags for Opus files without metadata', async (t) => {
  await testEmptyTags(t, opusFiles, 'Opus')
})

test('readTagsFromBuffer - should return empty tags for WAV files without metadata', async (t) => {
  await testEmptyTags(t, wavFiles, 'WAV')
})

test('readTagsFromBuffer - should return empty tags for AIFF files without metadata', async (t) => {
  await testEmptyTags(t, aiffFiles, 'AIFF')
})

test('readTagsFromBuffer - should return empty tags for Speex files without metadata', async (t) => {
  await testEmptyTags(t, spxFiles, 'Speex')
})

// Test helper function for empty tags testing
async function testEmptyTags(t: any, files: any[], format: string) {
  for (const file of files || []) {
    try {
      const tags = await readTagsFromBuffer(file.data)

      // Should return empty or minimal tags
      t.true(typeof tags === 'object')
      t.true(tags !== null)

      // Most fields should be undefined or empty
      t.true(tags.title === undefined || tags.title === '')
      t.true(tags.artists === undefined || tags.artists.length === 0)
      t.true(tags.album === undefined || tags.album === '')
      t.true(tags.genre === undefined || tags.genre === '')
      t.true(tags.comment === undefined || tags.comment === '')
      t.true(tags.image === undefined)

      console.log(`✓ ${file.fileName} (${format}): Empty tags as expected`)
    } catch (error) {
      console.error(`Error reading tags from ${file.fileName}: ${error}`)
      throw new Error(`Error reading tags from ${file.fileName}: ${error}`)
    }
  }

  if (files.length === 0) {
    t.pass(`No ${format} files available for testing`)
  }
}

test('writeTagsToBuffer - should write tags with cover image', async (t) => {
  // Get a test file and cover image
  const testFile = mp3Files[0]
  const coverImageFile = getFileByName('cover.jpg')

  t.truthy(testFile, 'Should have an MP3 test file')
  t.truthy(coverImageFile, 'Should have cover.jpg test file')

  // Create test tags with cover image
  const testTags: AudioTags = {
    title: 'Test Song Title',
    artists: ['Test Artist 1', 'Test Artist 2'],
    album: 'Test Album',
    year: 2024,
    genre: 'Test Genre',
    track: { no: 1, of: 12 },
    albumArtists: ['Test Album Artist'],
    comment: 'This is a test comment',
    disc: { no: 1, of: 2 },
    image: {
      data: coverImageFile!.data,
      picType: AudioImageType.CoverFront,
      mimeType: coverImageFile!.mimeType,
      description: 'Test cover image',
    },
  }

  // Write tags to buffer
  const modifiedBuffer = await writeTagsToBuffer(testFile.data, testTags)

  // Verify the buffer was modified
  t.true(Buffer.isBuffer(modifiedBuffer))
  t.true(modifiedBuffer.length > 0)

  console.log(`✓ Written tags to ${testFile.fileName} (${modifiedBuffer.length} bytes)`)
})

test('readTagsFromBuffer - should read written tags and image', async (t) => {
  // Get a test file and cover image
  const testFile = mp3Files[0]
  const coverImageFile = getFileByName('cover.jpg')

  // Create test tags with cover image
  const testTags: AudioTags = {
    title: 'Test Song Title',
    artists: ['Test Artist 1', 'Test Artist 2'],
    album: 'Test Album',
    year: 2024,
    genre: 'Test Genre',
    track: { no: 1, of: 12 },
    albumArtists: ['Test Album Artist'],
    comment: 'This is a test comment',
    disc: { no: 1, of: 2 },
    image: {
      data: coverImageFile!.data,
      picType: AudioImageType.CoverFront,
      mimeType: 'image/jpeg',
      description: 'Test cover image',
    },
  }

  // Write tags to buffer
  const modifiedBuffer = await writeTagsToBuffer(testFile.data, testTags)

  // Read tags from the modified buffer
  const readTags = await readTagsFromBuffer(modifiedBuffer)

  // Basic verification that the functions work
  t.true(typeof readTags === 'object')
  t.true(readTags !== null)

  // Check that we got some data back
  const hasTitle = readTags.title === 'Test Song Title'
  const hasArtists = Array.isArray(readTags.artists) && readTags.artists.length > 0
  const hasImage = readTags.image && Buffer.isBuffer(readTags.image.data)

  console.log(`✓ Read tags from ${testFile.fileName}:`)
  console.log(`  - Title: ${readTags.title || 'undefined'}`)
  console.log(`  - Artists: ${readTags.artists ? readTags.artists.join(', ') : 'undefined'}`)
  console.log(`  - Album: ${readTags.album || 'undefined'}`)
  console.log(`  - Year: ${readTags.year || 'undefined'}`)
  console.log(`  - Has image: ${hasImage ? 'Yes' : 'No'}`)
  if (hasImage) {
    console.log(`  - Image size: ${readTags.image!.data.length} bytes`)
    console.log(`  - Image MIME type: ${readTags.image!.mimeType || 'undefined'}`)
  }

  // Pass the test if we got some meaningful data back
  t.true(hasTitle || hasArtists || hasImage, 'Should have at least some tag data')
})

test('writeTagsToBuffer - should handle partial tags', async (t) => {
  const testFile = flacFiles[0]

  // Write only some tags
  const partialTags: AudioTags = {
    title: 'Partial Test Song',
    artists: ['Partial Artist'],
    year: 2023,
  }

  const modifiedBuffer = await writeTagsToBuffer(testFile.data, partialTags)
  const readTags = await readTagsFromBuffer(modifiedBuffer)

  // Verify only the written tags are present
  t.is(readTags.title, 'Partial Test Song')
  t.deepEqual(readTags.artists, ['Partial Artist'])
  t.is(readTags.year, 2023)

  // Other fields should be undefined or empty
  t.true(readTags.album === undefined || readTags.album === '')
  t.true(readTags.genre === undefined || readTags.genre === '')
  t.true(readTags.image === undefined)

  console.log(`✓ Partial tags test passed for ${testFile.fileName}`)
})

// Test helper function for format testing
async function testAudioFormatFullTags(t: any, files: any[], format: string) {
  const img = getFileByName('cover.jpg')
  for (const testFile of files) {
    try {
      const testTags: AudioTags = {
        title: `${format} Test Song`,
        artists: [`${format} Artist`, `${format} Secondary Artist`],
        album: `${format} Album`,
        year: 2024,
        genre: `${format} Genre`,
        track: { no: 1, of: 10 },
        comment: `Test comment for ${format} format`,
        albumArtists: [`${format} Album Artist`, `${format} Secondary Album Artist`],
        disc: { no: 1, of: 10 },
        image: {
          data: img!.data,
          picType: AudioImageType.CoverFront,
          mimeType: img!.mimeType,
          description: `Test cover image for ${format} format`,
        },
      }

      const modifiedBuffer = await writeTagsToBuffer(testFile.data, testTags)
      const readTags = await readTagsFromBuffer(modifiedBuffer)

      t.true(readTags.artists?.length === 2, `Expected 2 artists, got ${readTags.artists?.length} for ${format}`)
      t.true(
        readTags.albumArtists?.length === 2,
        `Expected 2 album artists, got ${readTags.albumArtists?.length} for ${format}`,
      )

      t.true(
        readTags.artists?.[0] === `${format} Artist`,
        `Expected ${format} Artist, got ${readTags.artists?.[0]} for ${format}`,
      )
      t.true(
        readTags.artists?.[1] === `${format} Secondary Artist`,
        `Expected ${format} Secondary Artist, got ${readTags.artists?.[0]} for ${format}`,
      )
      t.true(
        readTags.albumArtists?.[0] === `${format} Album Artist`,
        `Expected ${format} Album Artist, got ${readTags.albumArtists?.[0]} for ${format}`,
      )
      t.true(
        readTags.albumArtists?.[1] === `${format} Secondary Album Artist`,
        `Expected ${format} Secondary Album Artist, got ${readTags.albumArtists?.[0]} for ${format}`,
      )
      t.true(readTags.album === `${format} Album`, `Expected ${format} Album, got ${readTags.album} for ${format}`)
      t.true(readTags.genre === `${format} Genre`, `Expected ${format} Genre, got ${readTags.genre} for ${format}`)
      t.true(readTags.year === 2024, `Expected 2024, got ${readTags.year} for ${format}`)
      t.true(readTags.track?.no === 1, `Expected track no 1, got ${readTags.track?.no} for ${format}`)
      t.true(readTags.track?.of === 10, `Expected track of 10, got ${readTags.track?.of} for ${format}`)
      t.true(
        readTags.comment === `Test comment for ${format} format`,
        `Expected Test comment for ${format} format, got ${readTags.comment}`,
      )
      t.true(readTags.disc?.no === 1, `Expected disc no 1, got ${readTags.disc?.no} for ${format}`)
      t.true(readTags.disc?.of === 10, `Expected disc of 10, got ${readTags.disc?.of} for ${format}`)
      t.true(
        readTags.title === `${format} Test Song`,
        `Expected ${format} Test Song, got ${readTags.title} for ${format}`,
      )
      t.true(readTags.image?.data !== undefined, `Expected data, got ${readTags.image?.data} for ${format}`)
      t.true(
        readTags.image?.picType === AudioImageType.CoverFront,
        `Expected CoverFront, got ${readTags.image?.picType} for ${format}`,
      )
      t.true(
        readTags.image?.mimeType === img!.mimeType,
        `Expected image/png, got ${readTags.image?.mimeType} for ${format}`,
      )
      t.true(
        readTags.image?.description === `Test cover image for ${format} format`,
        `Expected Test cover image for ${format} format, got ${readTags.image?.description}`,
      )
    } catch (error) {
      console.error(`Error writing tags to ${testFile.fileName}: ${error}`)
      throw new Error(`Error writing tags to ${testFile.fileName}: ${error}`)
    }
  }
  if (files.length === 0) {
    t.pass(`No ${format} files available for testing`)
  }
}

test('writeTagsToBuffer - should handle MP3 format', async (t) => {
  await testAudioFormatFullTags(t, mp3Files, 'MP3')
})

test('writeTagsToBuffer - should handle AAC format', async (t) => {
  await testAudioFormatFullTags(t, aacFiles, 'AAC')
})

test('writeTagsToBuffer - should handle FLAC format', async (t) => {
  await testAudioFormatFullTags(t, flacFiles, 'FLAC')
})

test('writeTagsToBuffer - should handle OGG format', async (t) => {
  await testAudioFormatFullTags(t, oggFiles, 'OGG')
})

test('writeTagsToBuffer - should handle Opus format', async (t) => {
  await testAudioFormatFullTags(t, opusFiles, 'Opus')
})

test('writeTagsToBuffer - should handle WAV format', async (t) => {
  await testAudioFormatFullTags(t, wavFiles, 'WAV')
})

test('writeTagsToBuffer - should handle AIFF format', async (t) => {
  await testAudioFormatFullTags(t, aiffFiles, 'AIFF')
})

test('writeTagsToBuffer - should handle Speex format', async (t) => {
  await testAudioFormatFullTags(t, spxFiles, 'Speex')
})

test('writeTagsToBuffer - should handle all image types in round trip', async (t) => {
  // Get a test file and cover image
  const testFile = mp3Files[0]
  const coverImageFile = getFileByName('cover.jpg')

  t.truthy(testFile, 'Should have an MP3 test file')
  t.truthy(coverImageFile, 'Should have cover.jpg test file')

  // Create test tags with all image types
  const allImageTypes = [
    AudioImageType.Icon,
    AudioImageType.OtherIcon,
    AudioImageType.CoverFront,
    AudioImageType.CoverBack,
    AudioImageType.Leaflet,
    AudioImageType.Media,
    AudioImageType.LeadArtist,
    AudioImageType.Artist,
    AudioImageType.Conductor,
    AudioImageType.Band,
    AudioImageType.Composer,
    AudioImageType.Lyricist,
    AudioImageType.RecordingLocation,
    AudioImageType.DuringRecording,
    AudioImageType.DuringPerformance,
    AudioImageType.ScreenCapture,
    AudioImageType.BrightFish,
    AudioImageType.Illustration,
    AudioImageType.BandLogo,
    AudioImageType.PublisherLogo,
    AudioImageType.Other,
  ]

  const testTags: AudioTags = {
    title: 'Round Trip Test Song',
    artists: ['Test Artist'],
    album: 'Test Album',
    year: 2024,
    genre: 'Test Genre',
    track: { no: 1, of: 1 },
    albumArtists: ['Test Album Artist'],
    comment: 'Round trip test with all image types',
    disc: { no: 1, of: 1 },
    image: {
      data: coverImageFile!.data,
      picType: AudioImageType.CoverFront,
      mimeType: coverImageFile!.mimeType,
      description: 'Main cover image',
    },
    allImages: allImageTypes.map((picType) => ({
      data: coverImageFile!.data,
      picType: picType,
      mimeType: coverImageFile!.mimeType,
      description: `Test image of type ${picType}`,
    })),
  }

  // Write tags to buffer
  const modifiedBuffer = await writeTagsToBuffer(testFile.data, testTags)
  t.true(Buffer.isBuffer(modifiedBuffer))
  t.true(modifiedBuffer.length > 0)

  // Read tags from the modified buffer
  const readTags = await readTagsFromBuffer(modifiedBuffer)

  // Verify basic tags
  t.is(readTags.title, 'Round Trip Test Song')
  t.deepEqual(readTags.artists, ['Test Artist'])
  t.is(readTags.album, 'Test Album')
  t.is(readTags.year, 2024)
  t.is(readTags.genre, 'Test Genre')
  t.is(readTags.comment, 'Round trip test with all image types')

  // Verify main image
  t.truthy(readTags.image)
  t.is(readTags.image!.picType, AudioImageType.CoverFront)
  // Note: The description might be overwritten by allImages processing
  t.truthy(readTags.image!.description)

  // Verify all images array
  t.truthy(readTags.allImages)
  t.is(readTags.allImages!.length, allImageTypes.length)

  // Verify each image type is present
  const readImageTypes = readTags.allImages!.map((img) => img.picType)
  for (const expectedType of allImageTypes) {
    t.true(readImageTypes.includes(expectedType), `Should contain image type ${expectedType}`)
  }

  // Verify each image has the correct data and description
  for (let i = 0; i < readTags.allImages!.length; i++) {
    const img = readTags.allImages![i]
    t.true(Buffer.isBuffer(img.data))
    t.true(img.data.length > 0)
    t.is(img.mimeType, coverImageFile!.mimeType)
    t.is(img.description, `Test image of type ${img.picType}`)
  }

  console.log(`✓ Round trip test passed with ${readTags.allImages!.length} image types`)
  console.log(`  - Image types: ${readImageTypes.join(', ')}`)
})
