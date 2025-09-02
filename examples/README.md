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

- `title` - Song title
- `artists` - Array of primary artists
- `album` - Album name
- `year` - Release year
- `genre` - Music genre
- `track` - Track position object with `no` (current) and `of` (total)
- `albumArtists` - Array of album artists
- `comment` - Additional comments
- `disc` - Disc position object with `no` (current) and `of` (total)

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
