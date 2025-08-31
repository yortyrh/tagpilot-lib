# Audio Tags Reader Example

This directory contains a simple example demonstrating how to use the `readTags` function from the tagpilot-lib package.

## Files

- **`read-tags-example.js`** - Simple example that reads a file and prints tags as JSON
- **`README.md`** - This documentation file

## Quick Start

```bash
# Read tags from an audio file
node examples/read-tags-example.js ./music/01.mp3

# Read tags from a different file
node examples/read-tags-example.js ./music/formats/flac/01.flac
```

## What the Example Does

The example is very simple:

1. Takes a file path as a command line argument
2. Reads the audio metadata using `readTags()`
3. Parses the JSON result
4. Prints the tags in a nicely formatted JSON structure

## Example Output

```json
{
  "title": "Finnegan's Wake",
  "artist": "The Dubliners",
  "album": "Celtic Moods",
  "year": 2002,
  "genre": null,
  "track": 3,
  "album_artist": "The Dubliners",
  "comment": null,
  "language": null,
  "disc": 3
}
```

## Supported Audio Formats

The `readTags` function supports various audio formats through the lofty library:

- MP3, FLAC, M4A, WAV, OGG, AIFF, WAVPack, and more

## Metadata Fields

Each audio file can return the following metadata:

- `title` - Song title
- `artist` - Main artist
- `album` - Album name
- `year` - Release year
- `genre` - Music genre
- `track` - Track number
- `album_artist` - Album artist
- `comment` - Comments/notes
- `language` - Language (usually null)
- `disc` - Disc number

## Error Handling

If something goes wrong, the example will:

- Show a usage message if no file path is provided
- Display the error message if the file can't be read
- Exit with an error code for proper error handling
