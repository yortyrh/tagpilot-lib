# Examples

This directory contains examples demonstrating how to use the `tagpilot-lib` library.

## Quick Start

1. **Read tags from an audio file:**

   ```bash
   node examples/read-tags-example.js ./music/01.mp3
   ```

2. **Write/modify tags in an audio file:**
   ```bash
   node examples/write-tags-example.js ./music/01.mp3
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
  "artist": "The Dubliners",
  "album": "Celtic Moods",
  "year": 2002,
  "track": 3,
  "trackTotal": 19,
  "albumArtist": "The Dubliners",
  "disc": 3,
  "discTotal": 3
}
```

### `write-tags-example.js`

**What it does:** Demonstrates how to modify audio file metadata by reading, updating, and writing tags.

**Features:**

- Reads existing tags from an audio file
- Modifies specific tag values (title, comment, year, genre)
- Writes the updated tags back to the file
- Verifies changes by reading the file again
- Shows a summary of what was changed

**Example modifications:**

- Adds `[MODIFIED]` prefix to title
- Appends modification note to comment
- Increments year by 1
- Sets a default genre if none exists

**Example output:**

```
=== Writing tags to: ./music/01.mp3 ===

1. Reading original tags...
Original tags: { ... }

2. Creating modified tags...
Modified tags: { ... }

3. Writing modified tags to file...
✅ Tags written successfully!

4. Verifying changes...
Updated tags: { ... }

5. Summary of changes:
   Title: "Finnegan's Wake" → "[MODIFIED] Finnegan's Wake"
   Comment: null → "Modified by writeTags example"
   Year: 2002 → 2003
   Genre: null → "Modified Genre"

✅ Tag modification completed successfully!
```

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
- `artist` - Primary artist
- `album` - Album name
- `year` - Release year
- `genre` - Music genre
- `track` - Track number
- `trackTotal` - Total number of tracks
- `albumArtist` - Album artist
- `comment` - Additional comments
- `disc` - Disc number
- `discTotal` - Total number of discs

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
