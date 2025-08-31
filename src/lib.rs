#![deny(clippy::all)]

use lofty::file::{AudioFile};
use lofty::prelude::TaggedFileExt;
use lofty::config::WriteOptions;
use lofty::{read_from_path};
use lofty::tag::{Accessor, ItemKey, Tag};
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
pub async fn read_tags(file_path: String) -> Result<AudioTags> {
  let path = Path::new(&file_path);

  match read_from_path(path) {
    Ok(tagged_file) => {
      let tag = tagged_file.primary_tag();
      
      if tag.is_none() {
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
      
      Ok(AudioTags {
        title: tag.unwrap().title().map(|s| s.to_string()),
        artist: tag.unwrap().artist().map(|s| s.to_string()),
        album: tag.unwrap().album().map(|s| s.to_string()),
        year: tag.unwrap().year(),
        genre: tag.unwrap().genre().map(|s| s.to_string()),
        track: tag.unwrap().track(),
        track_total: tag.unwrap().track_total(),
        album_artist: tag.unwrap().artist().map(|s| s.to_string()),
        comment: tag.unwrap().comment().map(|s| s.to_string()),
        disc: tag.unwrap().disk(),
        disc_total: tag.unwrap().disk_total(),
      })
    }
    Err(e) => Err(napi::Error::from_reason(format!(
      "Failed to read audio file: {}",
      e
    ))),
  }
}

#[napi]
pub async fn write_tags(file_path: String, tags: AudioTags) -> Result<()> {
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
