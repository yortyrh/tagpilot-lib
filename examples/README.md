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

Reads audio file metadata and displays it as formatted JSON.

**Usage:**

```bash
node examples/read-tags-example.js ./music/01.mp3
```

**Output:**

```json
{
  "title": "Song Title",
  "artists": ["Artist Name"],
  "album": "Album Name",
  "year": 2024,
  "track": { "no": 1, "of": 12 },
  "image": { "data": "<Buffer>", "mimeType": "image/jpeg" }
}
```

### `write-tags-example.js`

Writes sample metadata tags to an audio file.

**Usage:**

```bash
node examples/write-tags-example.js ./music/01.mp3
```

**Features:**

- Creates comprehensive sample tag data
- Writes tags to the file
- Verifies changes by reading the file again

### `clear-tags-example.js`

Clears all metadata tags from an audio file.

**Usage:**

```bash
node examples/clear-tags-example.js ./music/01.mp3
```

**Features:**

- Shows existing tags before clearing
- Removes all metadata
- Verifies tags have been cleared

### `read-tags-from-buffer-example.js`

Reads audio file tags from a buffer instead of a file path.

**Usage:**

```bash
node examples/read-tags-from-buffer-example.js ./music/01.mp3
```

**Use cases:**

- Processing audio data from network requests
- Working with audio data in memory
- Avoiding file system I/O for better performance

### `write-tags-to-buffer-example.js`

Writes audio file tags to a buffer instead of directly to a file.

**Usage:**

```bash
node examples/write-tags-to-buffer-example.js ./music/01.mp3
```

**Use cases:**

- Processing audio data in memory without file I/O
- Working with audio streams and buffers
- Web applications that handle audio uploads

### `read-cover-image-example.js`

Reads cover images from audio files and saves them in multiple formats.

**Usage:**

```bash
node examples/read-cover-image-example.js ./music/01.mp3
```

**Features:**

- Extracts cover art from audio files
- Automatically detects image MIME type
- Saves as both data URL and image file

### `cover-image-example.js`

Adds cover images to audio files and saves the modified files.

**Usage:**

```bash
node examples/cover-image-example.js ./music/01.mp3 ./cover.jpg
```

**Features:**

- Embeds cover image into audio file
- Saves modified audio file with new name
- Shows file size changes

### `cover-image-buffer-example.js`

Advanced cover image operations using buffer-based processing.

**Usage:**

```bash
node examples/cover-image-buffer-example.js ./music/01.mp3 ./cover.jpg
```

**Features:**

- Reads and writes cover images using buffers
- Verifies cover image operations
- Compares file sizes before and after

## Supported Audio Formats

- MP3, M4A, FLAC, WAV, OGG, AAC, AIFF, OPUS, Speex, WavPack

## Metadata Fields

- **Basic:** `title`, `artists`, `album`, `year`, `genre`, `comment`
- **Position:** `track` (no/of), `disc` (no/of)
- **Artists:** `albumArtists`
- **Cover Art:** `image` (data, mimeType, description)

## Usage Notes

- Use relative or absolute paths to audio files
- Ensure read/write permissions for target files
- Consider backing up files before modifying tags
