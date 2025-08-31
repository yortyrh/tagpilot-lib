#![deny(clippy::all)]

use lofty::file::{AudioFile, FileType};
use lofty::prelude::TaggedFileExt;
use lofty::config::WriteOptions;
use lofty::probe::Probe;
use lofty::{read_from_path};
use lofty::tag::{Accessor, Tag, TagType};
use napi::Result;
use napi_derive::napi;
use serde::Serialize;
use std::path::Path;

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct AudioTags {
  pub title: Option<String>,
  pub artist: Option<String>,
  pub album: Option<String>,
  pub year: Option<u32>,
  pub genre: Option<String>,
  pub track: Option<u32>,       // Changed back to u32 to use track() function
  pub track_total: Option<u32>, // Total number of tracks
  pub album_artist: Option<String>,
  pub comment: Option<String>,
  pub disc: Option<u32>,
  pub disc_total: Option<u32>, // Total number of discs
}

#[napi]
pub fn read_tags(file_path: String) -> Result<AudioTags> {
  let path = Path::new(&file_path);

  match read_from_path(path) {
    Ok(tagged_file) => {
      if let Some(tag) = tagged_file.primary_tag() {
        Ok(AudioTags {
          title: tag.title().map(|s| s.to_string()),
          artist: tag.artist().map(|s| s.to_string()),
          album: tag.album().map(|s| s.to_string()),
          year: tag.year(),
          genre: tag.genre().map(|s| s.to_string()),
          track: tag.track(),
          track_total: tag.track_total(),
          album_artist: tag.artist().map(|s| s.to_string()),
          comment: tag.comment().map(|s| s.to_string()),
          disc: tag.disk(),
          disc_total: tag.disk_total(),
        })
      } else {
        Err(napi::Error::from_reason(format!(
          "No tag found",
        )))
      }
    }
    Err(e) => Err(napi::Error::from_reason(format!(
      "Failed to read audio file: {}",
      e
    ))),
  }
}

#[napi]
pub fn write_tags(file_path: String, tags: AudioTags) -> Result<()> {
  let path = Path::new(&file_path);
  // Read the existing file
  let mut tagged_file = match read_from_path(path) {
    Ok(tf) => tf,
    Err(e) => {
      return Err(napi::Error::from_reason(format!(
        "Failed to read audio file: {}",
        e
      )))
    }
  };

  // Get or create the primary tag
  if tagged_file.primary_tag().is_none() {
    // get the tag to create
    // AAC (ADTS) -> ID3v2
    // Ape -> APE
    // AIFF -> ID3v2
    // FLAC -> Vorbis Comments
    // MP3 -> ID3v2
    // MP4 -> iTunes-style ilst
    // MPC -> APE
    // Opus -> Vorbis Comments
    // Ogg Vorbis -> Vorbis Comments
    // Speex -> Vorbis Comments
    // WAV -> ID3v2
    // WavPack -> APE
    let tagged_file2 = Probe::open(path).unwrap().guess_file_type().unwrap().read().unwrap();
    let format = tagged_file2.file_type();

    let tag_type = match format {
      FileType::Ape => TagType::Ape,
      FileType::Flac => TagType::VorbisComments,
      FileType::Mp4 => TagType::Mp4Ilst,
      FileType::Mpc => TagType::Ape,
      FileType::Opus => TagType::VorbisComments,
      FileType::Vorbis => TagType::VorbisComments,
      FileType::Speex => TagType::VorbisComments,
      FileType::Wav => TagType::Id3v2,
      FileType::WavPack => TagType::Ape,
      _ => TagType::Id3v2,
    };
    tagged_file.insert_tag(Tag::new(tag_type));
  }

  // Check if the format is supported
  let has_tags = !tagged_file.tags().is_empty() || tagged_file.primary_tag().is_some();
  if !has_tags {
    return Err(napi::Error::from_reason(
      "File format not supported by lofty",
    ));
  }

  // Get the primary tag or create a new one
  let mut primary_tag = tagged_file.primary_tag().cloned().unwrap_or_else(|| {
    use lofty::tag::Tag;
    Tag::new(lofty::tag::TagType::Id3v2)
  });

  // Update the tag with new values
  if let Some(title) = tags.title {
    primary_tag.set_title(title);
  }
  if let Some(artist) = tags.artist {
    primary_tag.set_artist(artist);
  }
  if let Some(album) = tags.album {
    primary_tag.set_album(album);
  }
  if let Some(year) = tags.year {
    primary_tag.set_year(year);
  }
  if let Some(genre) = tags.genre {
    primary_tag.set_genre(genre);
  }
  if let Some(track) = tags.track {
    primary_tag.set_track(track);
  }
  if let Some(track_total) = tags.track_total {
    primary_tag.set_track_total(track_total);
  }
  if let Some(album_artist) = tags.album_artist {
    primary_tag.set_artist(album_artist);
  }
  if let Some(comment) = tags.comment {
    primary_tag.set_comment(comment);
  }
  if let Some(disc) = tags.disc {
    primary_tag.set_disk(disc);
  }
  if let Some(disc_total) = tags.disc_total {
    primary_tag.set_disk_total(disc_total);
  }

  // Write the updated tag back to the file
  match tagged_file.save_to_path(path, WriteOptions::default()) {
    Ok(_) => Ok(()),
    Err(e) => Err(napi::Error::from_reason(format!(
      "Failed to write audio file: {}",
      e
    ))),
  }
}
