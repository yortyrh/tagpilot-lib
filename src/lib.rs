#![deny(clippy::all)]

use lofty::file::{AudioFile};
use lofty::prelude::TaggedFileExt;
use lofty::config::WriteOptions;
use lofty::probe::Probe;
use lofty::tag::{Accessor, ItemKey, Tag};
use napi::bindgen_prelude::Buffer;
use napi::Result;
use napi_derive::napi;
use serde::Serialize;
use std::path::Path;
use std::io::{Cursor, Read};

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
pub async fn read_tags(file_path: String) -> Result<AudioTags> {
  let path = Path::new(&file_path);

  match Probe::open(path).unwrap().guess_file_type().unwrap().read() {
    Ok(tagged_file) => {
      let tag = tagged_file.primary_tag();
      match tag {
        Some(tag) => {
          return Ok(AudioTags {
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
          });
        }
        None => {
          return Ok(AudioTags {
            title: None,
            artist: None,
            album: None,
            year: None,
            genre: None,
            track: None,
            track_total: None,
            album_artist: None,
            comment: None,
            disc: None,
            disc_total: None,
          });
        }
      }
    }
    Err(e) => Err(napi::Error::from_reason(format!(
      "Failed to read audio file: {}",
      e
    ))),
  }
}

#[napi]
pub async fn read_tags_from_buffer(buffer: napi::bindgen_prelude::Buffer) -> Result<AudioTags> {
  let buffer_ref = buffer.as_ref();
  let mut cursor = Cursor::new(buffer_ref);
  
  match Probe::new(&mut cursor).guess_file_type().unwrap().read() {
    Ok(tagged_file) => {
      let tag = tagged_file.primary_tag();
      match tag {
        Some(tag) => {
          return Ok(AudioTags {
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
          });
        }
        None => {
          return Ok(AudioTags {
            title: None,
            artist: None,
            album: None,
            year: None,
            genre: None,
            track: None,
            track_total: None,
            album_artist: None,
            comment: None,
            disc: None,
            disc_total: None,
          });
        }
      }
    }
    Err(e) => Err(napi::Error::from_reason(format!(
      "Failed to read audio from buffer: {}",
      e
    ))),
  }
}

#[napi]
pub async fn write_tags(file_path: String, tags: AudioTags) -> Result<()> {
  let path = Path::new(&file_path);
  // Read the existing file
  let mut tagged_file = match Probe::open(path).unwrap().guess_file_type().unwrap().read() {
    Ok(tf) => tf,
    Err(e) => {
      return Err(napi::Error::from_reason(format!(
        "Failed to read audio file: {}",
        e
      )))
    }
  };

  // Check if the file has tags
  if tagged_file.primary_tag().is_none() {
    // create the principal tag
    let tag = Tag::new(tagged_file.primary_tag_type());
    tagged_file.insert_tag(tag);
  }

  let primary_tag = tagged_file.primary_tag_mut().unwrap();

  // Update the tag with new values
  if let Some(title) = tags.title {
    primary_tag.insert_text(ItemKey::TrackTitle, title);
  }
  if let Some(artist) = tags.artist {
    primary_tag.insert_text(ItemKey::TrackArtist, artist);
  }
  if let Some(album) = tags.album {
    primary_tag.insert_text(ItemKey::AlbumTitle, album);
  }
  if let Some(year) = tags.year {
    primary_tag.insert_text(ItemKey::Year, year.to_string());
  }
  if let Some(genre) = tags.genre {
    primary_tag.insert_text(ItemKey::Genre, genre);
  }
  if let Some(track) = tags.track {
    primary_tag.insert_text(ItemKey::TrackNumber, track.to_string());
  }
  if let Some(track_total) = tags.track_total {
    primary_tag.insert_text(ItemKey::TrackTotal, track_total.to_string());
  }
  if let Some(album_artist) = tags.album_artist {
    primary_tag.insert_text(ItemKey::AlbumArtist, album_artist);
  }
  if let Some(comment) = tags.comment {
    primary_tag.insert_text(ItemKey::Comment, comment);
  }
  if let Some(disc) = tags.disc {
    primary_tag.insert_text(ItemKey::DiscNumber, disc.to_string());
  }
  if let Some(disc_total) = tags.disc_total {
    primary_tag.insert_text(ItemKey::DiscTotal, disc_total.to_string());
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

#[napi]
pub async fn clear_tags(file_path: String) -> Result<()> {
  let path = Path::new(&file_path);
  
  // Read the existing file
  let mut tagged_file = match Probe::open(path).unwrap().guess_file_type().unwrap().read() {
    Ok(tf) => tf,
    Err(e) => {
      return Err(napi::Error::from_reason(format!(
        "Failed to read audio file: {}",
        e
      )))
    }
  };

  // Create a new empty tag of the same type
  let empty_tag = Tag::new(tagged_file.primary_tag_type());
  
  // Replace the existing primary tag with the empty one
  tagged_file.insert_tag(empty_tag);

  // Write the file back with the empty tag
  match tagged_file.save_to_path(path, WriteOptions::default()) {
    Ok(_) => Ok(()),
    Err(e) => Err(napi::Error::from_reason(format!(
      "Failed to write audio file: {}",
      e
    ))),
  }
}

#[napi]
pub async fn write_tags_to_buffer(buffer: napi::bindgen_prelude::Buffer, tags: AudioTags) -> Result<napi::bindgen_prelude::Buffer> {
  let owned_vec: napi::bindgen_prelude::Buffer = buffer.into();
  // copy the buffer to a new vec
  let owned_copy: Vec<u8> = owned_vec.to_vec();
  
  // Create a fresh cursor for reading
  let mut input_cursor = Cursor::new(&owned_copy);
  
  // Read the existing file from buffer
  let mut tagged_file = match Probe::new(input_cursor.by_ref()).guess_file_type().unwrap().read() {
    Ok(tf) => tf,
    Err(e) => {
      return Err(napi::Error::from_reason(format!(
        "Failed to read audio from buffer: {}",
        e
      )))
    }
  };

  // Check if the file has tags
  if tagged_file.primary_tag().is_none() {
    // create the principal tag
    let tag = Tag::new(tagged_file.primary_tag_type());
    tagged_file.insert_tag(tag);
  }
  let primary_tag = tagged_file.primary_tag_mut().unwrap();

  // Update the tag with new values
  if let Some(title) = tags.title {
    primary_tag.insert_text(ItemKey::TrackTitle, title);
  }
  if let Some(artist) = tags.artist {
    primary_tag.insert_text(ItemKey::TrackArtist, artist);
  }
  if let Some(album) = tags.album {
    primary_tag.insert_text(ItemKey::AlbumTitle, album);
  }
  if let Some(year) = tags.year {
    primary_tag.insert_text(ItemKey::Year, year.to_string());
  }
  if let Some(genre) = tags.genre {
    primary_tag.insert_text(ItemKey::Genre, genre);
  }
  if let Some(track) = tags.track {
    primary_tag.insert_text(ItemKey::TrackNumber, track.to_string());
  }
  if let Some(track_total) = tags.track_total {
    primary_tag.insert_text(ItemKey::TrackTotal, track_total.to_string());
  }
  if let Some(album_artist) = tags.album_artist {
    primary_tag.insert_text(ItemKey::AlbumArtist, album_artist);
  }
  if let Some(comment) = tags.comment {
    primary_tag.insert_text(ItemKey::Comment, comment);
  }
  if let Some(disc) = tags.disc {
    primary_tag.insert_text(ItemKey::DiscNumber, disc.to_string());
  }
  if let Some(disc_total) = tags.disc_total {
    primary_tag.insert_text(ItemKey::DiscTotal, disc_total.to_string());
  }

  // Write to a new buffer
  let mut cursor = Cursor::new(owned_copy);
  match tagged_file.save_to(&mut cursor, WriteOptions::default()) {
    Ok(_) => {
      let owned_copy_buffer = Buffer::from(cursor.into_inner());
      Ok(owned_copy_buffer)
    },
    Err(e) => Err(napi::Error::from_reason(format!(
      "Failed to write audio to buffer: {}",
      e
    ))),
  }
}
