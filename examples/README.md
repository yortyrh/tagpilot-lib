# Examples

This directory contains examples demonstrating how to use the `@yortyrh/tagpilot-lib` library.

## Quick Start

1. **Read tags from an audio file:**

   ```bash
   node examples/read-tags-example.js ./music/01.mp3
   ```

2. **Write sample tags to an audio file:**

   ```bash
   node examples/write-tags-example.js ./music/01.mp3
   ```

3. **Clear all tags from an audio file:**

   ```bash
   node examples/clear-tags-example.js ./music/01.mp3
   ```

4. **Read tags from a buffer:**
   ```bash
   node examples/read-tags-from-buffer-example.js ./music/01.mp3
   ```

## Examples Overview

### `read-tags-example.js`

**What it does:** Reads audio file metadata and displays it as formatted JSON.

**Features:**

- Takes a file path as a command line argument
- Reads all available metadata tags
- Displays the result as formatted JSON
- Handles errors gracefully

**Example output:**

```json
{
  "title": "Finnegan's Wake",
  "artists": ["The Dubliners"],
  "album": "Celtic Moods",
  "year": 2002,
  "track": {
    "no": 3,
    "of": 19
  },
  "albumArtists": ["The Dubliners"],
  "disc": {
    "no": 3,
    "of": 3
  }
}
```

### `write-tags-example.js`

**What it does:** Demonstrates how to write sample metadata tags to an audio file.

**Features:**

- Reads existing tags from an audio file (if any)
- Creates comprehensive sample tag data with all supported fields
- Writes the sample tags to the file
- Verifies changes by reading the file again
- Shows a summary of what was written

**Sample data written:**

- Title: "Sample Song Title"
- Artists: ["Sample Artist"]
- Album: "Sample Album"
- Year: 2024
- Genre: "Sample Genre"
- Track: 1 of 12
- Album Artists: ["Sample Album Artist"]
- Comment: "This is a sample comment for demonstration purposes"
- Disc: 1 of 1

**Example output:**

```
=== Writing sample tags to: ./music/01.mp3 ===

1. Reading original tags...
No tags found in the file

2. Creating sample tags...
Sample tags to write:
{
  "title": "Sample Song Title",
  "artists": ["Sample Artist"],
  "album": "Sample Album",
  "year": 2024,
  "genre": "Sample Genre",
  "track": {
    "no": 1,
    "of": 12
  },
  "albumArtists": ["Sample Album Artist"],
  "comment": "This is a sample comment for demonstration purposes",
  "disc": {
    "no": 1,
    "of": 1
  }
}

3. Writing sample tags to file...
✅ Sample tags written successfully!

4. Verifying changes...
Updated tags: { ... }

5. Summary of sample data written:
   Title: "Sample Song Title"
   Artists: ["Sample Artist"]
   Album: "Sample Album"
   Year: 2024
   Genre: "Sample Genre"
   Track: 1 of 12
   Album Artists: ["Sample Album Artist"]
   Comment: "This is a sample comment for demonstration purposes"
   Disc: 1 of 1

✅ Sample tags written successfully!
```

**Use cases:**

- Testing tag writing functionality
- Creating demo files with sample metadata
- Learning how to structure tag data
- Preparing files for testing other functions
- Demonstrating the library's capabilities

### `clear-tags-example.js`

**What it does:** Demonstrates how to completely clear all metadata tags from an audio file.

**Features:**

- Reads existing tags from an audio file to show what will be cleared
- Clears all metadata tags (title, artist, album, year, genre, etc.)
- Verifies that tags have been successfully cleared
- Provides clear feedback on the operation status

**Example output:**

```
=== Clearing tags from: ./music/01.mp3 ===

1. Reading original tags...
Original tags found:
{
  "title": "Finnegan's Wake",
  "artists": ["The Dubliners"],
  "album": "Celtic Moods"
}

2. Clearing all tags...
Tags cleared successfully!

3. Verifying tags have been cleared...
✓ All tags have been successfully cleared!
File now contains no metadata tags.

=== Operation completed ===
```

**Use cases:**

- Removing personal information from audio files before sharing
- Cleaning up corrupted or unwanted metadata
- Preparing files for distribution without metadata
- Testing tag writing functionality with clean files

### `read-tags-from-buffer-example.js`

**What it does:** Demonstrates how to read audio file tags from a buffer instead of a file path.

**Features:**

- Reads an audio file into a buffer
- Extracts tags from the buffer using `readTagsFromBuffer`
- Compares results with file-based reading
- Shows the difference between buffer and file approaches
- Demonstrates use cases for buffer-based processing

**Example output:**

```
=== Reading tags from buffer: ./music/01.mp3 ===

1. Reading file into buffer...
   File size: 2417171 bytes
   Buffer type: Buffer

2. Reading tags from buffer...
Tags found in buffer:
{
  "title": "Sample Song Title",
  "artists": ["Sample Artist"],
  "album": "Sample Album"
}

3. Reading tags directly from file for comparison...
Tags found in file:
{
  "title": "Sample Song Title",
  "artists": ["Sample Artist"],
  "album": "Sample Album"
}

4. Comparing results...
✅ Buffer and file reading produce the same result
✅ Tags are identical between buffer and file reading

=== Use Cases for Buffer Reading ===
• Processing audio data from network requests
• Working with audio data in memory
• Processing audio streams
• Avoiding file system I/O for better performance
• Working with audio data from databases or cloud storage
```

**Use cases:**

- Processing audio data from network requests
- Working with audio data already in memory
- Processing audio streams without saving to disk
- Avoiding file system I/O for better performance
- Working with audio data from databases or cloud storage
- Processing audio data in web applications
- Handling audio uploads in web services

### `read-cover-image-example.js`

**What it does:** Demonstrates how to read cover images from audio files and save them in multiple formats.

**Features:**

- Reads cover image from an audio buffer using `readCoverImageFromBuffer`
- Automatically detects image MIME type (JPEG, PNG, GIF, BMP, TIFF)
- Converts cover image to data URL for web use
- Saves both data URL and cover image file
- Provides detailed file size and format information

**Files Saved:**

- **Data URL file**: `{audio-file}-cover-dataurl.txt` - Base64 encoded data URL
- **Cover image file**: `{audio-file}-cover.{format}` - Original image format

**Example output:**

```
=== Reading Cover Image as Data URL ===
Audio file: ./test-files/01-with-cover.flac

Reading audio file...
   ✅ Audio loaded: 34036419 bytes
Reading cover image from buffer...
   ✅ Cover image found: 2976200 bytes
   📋 Detected MIME type: image/jpeg
Converting to data URL...
   ✅ Data URL generated (3968291 characters)

💾 Saving data URL to file...
   ✅ Data URL saved: ./test-files/01-with-cover-cover-dataurl.txt
   📁 File size: 3875.28 KB
💾 Saving cover image as separate file...
   ✅ Cover image saved: ./test-files/01-with-cover-cover.jpeg
   📁 File size: 2906.45 KB
   🖼️  Format: image/jpeg

=== Files Saved ===
📄 Data URL: ./test-files/01-with-cover-cover-dataurl.txt
🖼️  Cover Image: ./test-files/01-with-cover-cover.jpeg
📊 Total saved: 2 files
```

**Use cases:**

- Extracting cover art for web applications
- Creating data URLs for HTML/CSS embedding
- Saving cover images as separate files
- Processing audio files in batch operations
- Web service APIs that need cover art
- Using `readCoverImageFromBuffer` for buffer-based operations

### `cover-image-example.js`

**What it does:** Demonstrates how to add cover images to audio files and save the modified files.

**Features:**

- Reads audio and image files into buffers
- Embeds cover image into audio file using `writeCoverImageToBuffer`
- Saves modified audio file with new name
- Shows file size changes and processing summary

**Example output:**

```
=== Setting Cover Image ===
Audio file: ./test-files/01.flac
Image file: ./music/01.mp3
Output file: ./test-files/01-with-cover.flac

Reading audio file...
   ✅ Audio loaded: 31110511 bytes
Reading image file...
   ✅ Image loaded: 2976200 bytes
Setting cover image...
   ✅ Cover image set successfully!
   Modified audio size: 34036419 bytes

💾 Saving modified audio file...
   ✅ File saved: ./test-files/01-with-cover.flac
   📁 File size: 33238.69 KB
   📊 Size change: 2857.33 KB

=== Files Summary ===
🎵 Original audio: ./test-files/01.flac
🖼️  Cover image: ./music/01.mp3
💾 Output audio: ./test-files/01-with-cover.flac
📊 Total processed: 3 files
```

**Use cases:**

- Adding cover art to audio files
- Batch processing audio files with cover images
- Creating audio files with embedded artwork
- Preparing files for distribution
- Using `writeCoverImageToBuffer` for buffer-based operations

### `cover-image-buffer-example.js`

**What it does:** Demonstrates advanced cover image operations using buffer-based processing.

**Features:**

- Reads existing cover images from audio buffers using `readCoverImageFromBuffer`
- Writes new cover images to audio buffers using `writeCoverImageToBuffer`
- Verifies cover image operations
- Compares file sizes before and after
- Saves modified audio files

**Example output:**

```
=== Cover Image Buffer Operations ===
Audio file: ./test-files/02.flac
Image file: ./music/02.mp3

Reading audio file...
   ✅ Audio loaded: 41045833 bytes
Reading image file...
   ✅ Image loaded: 7404054 bytes
Reading existing cover image from buffer...
   ℹ️  No existing cover image found

Writing cover image to buffer...
   ✅ Cover image written to buffer
   Modified audio size: 48449933 bytes
Verifying cover image was written...
   ✅ Cover image found in modified buffer: 7404054 bytes

💾 Saving modified buffer to file...
   ✅ File saved: ./test-files/02-with-cover.flac
   📁 File size: 47314.39 KB
   📊 Size change: 7230.57 KB

=== Files Summary ===
🎵 Original audio: ./test-files/02.flac
🖼️  Cover image: ./music/02.mp3
💾 Output audio: ./test-files/02-with-cover.flac
📊 Total processed: 3 files
```

**Use cases:**

- Advanced cover image processing
- Buffer-based audio manipulation
- Verifying cover image operations
- Processing audio streams in memory
- Using both `readCoverImageFromBuffer` and `writeCoverImageToBuffer`

## Supported Audio Formats

The library supports reading and writing tags for various audio formats including:

- MP3
- M4A
- FLAC
- WAV
- OGG
- And more (as supported by the `lofty` crate)

## Metadata Fields

Both examples work with the following metadata fields:

### Basic Information

- `title` - Song title
- `artists` - Array of primary artists
- `album` - Album name
- `year` - Release year
- `genre` - Music genre
- `comment` - Additional comments

### Position Information

- `track` - Track position object with `no` (current) and `of` (total)
- `disc` - Disc position object with `no` (current) and `of` (total)

### Artist Information

- `albumArtists` - Array of album artists

### Cover Art

- `image` - Cover image object with data, MIME type, and description

## Error Handling

Both examples include comprehensive error handling:

- File not found errors
- Unsupported format errors
- Read/write permission errors
- Invalid tag data errors

## Usage Notes

- **File paths:** Use relative or absolute paths to audio files
- **Permissions:** Ensure you have read/write permissions for the target files
- **Backup:** Consider backing up files before modifying tags
- **Formats:** Different audio formats may support different tag types and fields
