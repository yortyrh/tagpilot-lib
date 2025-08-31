#![deny(clippy::all)]

use lofty::prelude::TaggedFileExt;
use lofty::probe::Probe;
use lofty::tag::Accessor;
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

  // Probe the file to determine its format
  let tagged_file = match Probe::open(path) {
    Ok(probe) => match probe.read() {
      Ok(tagged_file) => tagged_file,
      Err(e) => {
        return Err(napi::Error::from_reason(format!(
          "Failed to read file: {}",
          e
        )));
      }
    },
    Err(e) => {
      return Err(napi::Error::from_reason(format!(
        "Failed to open file: {}",
        e
      )));
    }
  };

  // Check if the format is supported by checking if we have any tags or file type
  let has_tags = !tagged_file.tags().is_empty() || tagged_file.primary_tag().is_some();
  if !has_tags {
    return Err(napi::Error::from_reason(
      "File format not supported by lofty",
    ));
  }

  // Get all available tags and primary tag
  let all_tags = tagged_file.tags();
  let primary_tag = tagged_file.primary_tag();

  // Try to get track number and track total from different sources using track() and track_total() functions
  let mut track_number = None;
  let mut track_total = None;
  let mut disc_total = None;

  // First try primary tag
  if let Some(tag) = primary_tag {
    track_number = tag.track();
    track_total = tag.track_total();
    disc_total = tag.disk_total();
  }

  // If no track number in primary tag, try all available tags
  if track_number.is_none() || track_total.is_none() || disc_total.is_none() {
    for tag in all_tags.iter() {
      if track_number.is_none() {
        track_number = tag.track();
      }
      if track_total.is_none() {
        track_total = tag.track_total();
      }
      if disc_total.is_none() {
        disc_total = tag.disk_total();
      }

      // Break if we found all the values we need
      if track_number.is_some() && track_total.is_some() && disc_total.is_some() {
        break;
      }
    }
  }

  // Extract tags from the primary tag (usually the first one)
  let tags = if let Some(tag) = primary_tag {
    AudioTags {
      title: tag.title().map(|s| s.to_string()),
      artist: tag.artist().map(|s| s.to_string()),
      album: tag.album().map(|s| s.to_string()),
      year: tag.year(),
      genre: tag.genre().map(|s| s.to_string()),
      track: track_number,      // Use track number from track() function
      track_total: track_total, // Use track total from track_total() function
      album_artist: tag.artist().map(|s| s.to_string()), // Using artist as album_artist
      comment: tag.comment().map(|s| s.to_string()),
      disc: tag.disk(),
      disc_total: disc_total, // Use disc total from disk_total() function
    }
  } else {
    // If no primary tag, try to get tags from any available tag
    if all_tags.is_empty() {
      return Err(napi::Error::from_reason("No tags found in the audio file"));
    }

    let first_tag = &all_tags[0];
    AudioTags {
      title: first_tag.title().map(|s| s.to_string()),
      artist: first_tag.artist().map(|s| s.to_string()),
      album: first_tag.album().map(|s| s.to_string()),
      year: first_tag.year(),
      genre: first_tag.genre().map(|s| s.to_string()),
      track: track_number,      // Use track number from track() function
      track_total: track_total, // Use track total from track_total() function
      album_artist: first_tag.artist().map(|s| s.to_string()), // Using artist as album_artist
      comment: first_tag.comment().map(|s| s.to_string()),
      disc: first_tag.disk(),
      disc_total: disc_total, // Use disc total from disk_total() function
    }
  };

  Ok(tags)
}
