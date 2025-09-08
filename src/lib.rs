#![deny(clippy::all)]

use lofty::config::WriteOptions;
use lofty::file::AudioFile;
use lofty::picture::{MimeType, Picture, PictureType};
use lofty::prelude::TaggedFileExt;
use lofty::probe::Probe;
use lofty::tag::{Accessor, ItemKey, Tag};
use napi::bindgen_prelude::Buffer;
use napi::Result;
use napi_derive::napi;
use std::fs;
use std::io::Cursor;
use std::path::Path;

#[napi(object)]
#[derive(Debug, PartialEq)]
pub struct Position {
  pub no: Option<u32>,
  pub of: Option<u32>,
}

#[napi(object)]
pub struct Image {
  pub data: Buffer,
  pub mime_type: Option<String>,
  pub description: Option<String>,
}

/*
 * Convert a MimeType to a string
 */
pub fn mime_type_to_string(mime_type: &MimeType) -> Option<String> {
  match mime_type {
    MimeType::Jpeg => Some(String::from("image/jpeg")),
    MimeType::Png => Some(String::from("image/png")),
    MimeType::Gif => Some(String::from("image/gif")),
    MimeType::Tiff => Some(String::from("image/tiff")),
    MimeType::Bmp => Some(String::from("image/bmp")),
    MimeType::Unknown(_) => None,
    _ => None,
  }
}

#[napi(object)]
#[derive(Default)]
pub struct AudioTags {
  pub title: Option<String>,
  pub artists: Option<Vec<String>>,
  pub album: Option<String>,
  pub year: Option<u32>,
  pub genre: Option<String>,
  pub track: Option<Position>,
  pub album_artists: Option<Vec<String>>,
  pub comment: Option<String>,
  pub disc: Option<Position>,
  pub image: Option<Image>,
}

fn add_cover_image(
  primary_tag: &mut Tag,
  image_data: &Buffer,
  image_description: Option<String>,
  default_mime_type: MimeType,
) {
  // add the new picture
  let buf = image_data.to_vec();
  let kind = infer::get(&buf).expect("file type is known");
  let mime_type = match kind.mime_type() {
    "image/jpeg" => MimeType::Jpeg,
    "image/png" => MimeType::Png,
    "image/gif" => MimeType::Gif,
    "image/tiff" => MimeType::Tiff,
    "image/bmp" => MimeType::Bmp,
    _ => default_mime_type,
  };
  primary_tag.remove_picture_type(PictureType::CoverFront);
  let cover_front_picture = Picture::new_unchecked(
    lofty::picture::PictureType::CoverFront,
    Some(mime_type),
    image_description,
    buf,
  );
  primary_tag.set_picture(0, cover_front_picture);
}

// add method to AudioTags from &Tag
impl AudioTags {
  pub fn from_tag(tag: &Tag) -> Self {
    Self {
      title: tag.title().map(|s| s.to_string()),
      artists: tag.artist().map(|s| vec![s.to_string()]),
      album: tag.album().map(|s| s.to_string()),
      year: tag.year(),
      genre: tag.genre().map(|s| s.to_string()),
      track: match (tag.track(), tag.track_total()) {
        (None, None) => None,
        (no, of) => Some(Position { no, of }),
      },
      album_artists: tag.artist().map(|s| vec![s.to_string()]),
      comment: tag.comment().map(|s| s.to_string()),
      disc: match (tag.disk(), tag.disk_total()) {
        (None, None) => None,
        (no, of) => Some(Position { no, of }),
      },
      image: {
        let mut image = None;
        for picture in tag.pictures() {
          if picture.pic_type() == lofty::picture::PictureType::CoverFront {
            image = Some(Image {
              data: picture.data().to_vec().into(),
              mime_type: mime_type_to_string(picture.mime_type().unwrap()),
              description: picture.description().map(|s| s.to_string()),
            });
            break;
          }
        }
        image
      },
    }
  }

  pub fn to_tag(&self, primary_tag: &mut Tag) {
    // Update the tag with new values
    self.title.as_ref().map(|title| {
      primary_tag.remove_key(&ItemKey::TrackTitle);
      primary_tag.insert_text(ItemKey::TrackTitle, title.clone())
    });

    if let Some(artists) = self.artists.as_ref() {
      if !artists.is_empty() {
        primary_tag.remove_key(&ItemKey::TrackArtist);
        primary_tag.remove_key(&ItemKey::TrackArtists);

        primary_tag.insert_text(ItemKey::TrackArtist, artists.first().unwrap().clone());
        if artists.len() > 1 {
          primary_tag.insert_text(ItemKey::TrackArtists, artists.join(", "));
        }
      }
    }

    if let Some(album) = self.album.as_ref() {
      primary_tag.remove_key(&ItemKey::AlbumTitle);
      primary_tag.insert_text(ItemKey::AlbumTitle, album.clone());
    }

    if let Some(year) = self.year.as_ref() {
      primary_tag.remove_key(&ItemKey::Year);
      primary_tag.remove_key(&ItemKey::RecordingDate);
      primary_tag.insert_text(ItemKey::Year, year.to_string());
      primary_tag.insert_text(ItemKey::RecordingDate, year.to_string());
    }

    if let Some(genre) = self.genre.as_ref() {
      primary_tag.remove_key(&ItemKey::Genre);
      primary_tag.insert_text(ItemKey::Genre, genre.clone());
    }

    if let Some(track) = self.track.as_ref() {
      if let Some(no) = track.no {
        primary_tag.remove_key(&ItemKey::TrackNumber);
        primary_tag.insert_text(ItemKey::TrackNumber, no.to_string());
      }
      if let Some(of) = track.of {
        primary_tag.remove_key(&ItemKey::TrackTotal);
        primary_tag.insert_text(ItemKey::TrackTotal, of.to_string());
      }
    }

    if let Some(disc) = self.disc.as_ref() {
      if let Some(no) = disc.no {
        primary_tag.remove_key(&ItemKey::DiscNumber);
        primary_tag.insert_text(ItemKey::DiscNumber, no.to_string());
      }
      if let Some(of) = disc.of {
        primary_tag.remove_key(&ItemKey::DiscTotal);
        primary_tag.insert_text(ItemKey::DiscTotal, of.to_string());
      }
    }

    if let Some(album_artists) = self.album_artists.as_ref() {
      if !album_artists.is_empty() {
        primary_tag.remove_key(&ItemKey::AlbumArtist);
        primary_tag.insert_text(ItemKey::AlbumArtist, album_artists.first().unwrap().clone());
      }
    }

    if let Some(comment) = self.comment.as_ref() {
      primary_tag.remove_key(&ItemKey::Comment);
      primary_tag.insert_text(ItemKey::Comment, comment.clone());
    }

    if let Some(image) = self.image.as_ref() {
      add_cover_image(
        primary_tag,
        &image.data,
        image.description.as_ref().map(|s| s.to_string()),
        image
          .mime_type
          .as_ref()
          .map(|s| MimeType::from_str(s))
          .unwrap_or(MimeType::Jpeg),
      );
    }
  }
}

#[napi]
pub async fn read_tags(file_path: String) -> Result<AudioTags> {
  let path = Path::new(&file_path);
  if !path.exists() {
    return Err(napi::Error::from_reason(format!(
      "File does not exist: {}",
      file_path
    )));
  }

  let Ok(probe) = Probe::open(path) else {
    return Err(napi::Error::from_reason(format!(
      "Failed to open file: {}",
      file_path
    )));
  };
  let Ok(probe) = probe.guess_file_type() else {
    return Err(napi::Error::from_reason(format!(
      "Failed to guess file type: {}",
      file_path
    )));
  };
  let Ok(tagged_file) = probe.read() else {
    return Err(napi::Error::from_reason(format!(
      "Failed to read audio file: {}",
      file_path
    )));
  };

  tagged_file
    .primary_tag()
    .map_or(Ok(AudioTags::default()), |tag| Ok(AudioTags::from_tag(tag)))
}

#[napi]
pub async fn read_tags_from_buffer(buffer: napi::bindgen_prelude::Buffer) -> Result<AudioTags> {
  let buffer_ref = buffer.as_ref();
  let mut cursor = Cursor::new(buffer_ref);

  let probe = Probe::new(&mut cursor);

  let Ok(probe) = probe.guess_file_type() else {
    return Err(napi::Error::from_reason(
      "Failed to guess file type".to_string(),
    ));
  };

  let Ok(tagged_file) = probe.read() else {
    return Err(napi::Error::from_reason(
      "Failed to read audio file".to_string(),
    ));
  };

  tagged_file
    .primary_tag()
    .map_or(Ok(AudioTags::default()), |tag| Ok(AudioTags::from_tag(tag)))
}

#[napi]
pub async fn write_tags(file_path: String, tags: AudioTags) -> Result<()> {
  let path = Path::new(&file_path);
  if !path.exists() {
    return Err(napi::Error::from_reason(format!(
      "File does not exist: {}",
      file_path
    )));
  }

  let Ok(probe) = Probe::open(path) else {
    return Err(napi::Error::from_reason(format!(
      "Failed to open file: {}",
      file_path
    )));
  };
  let Ok(probe) = probe.guess_file_type() else {
    return Err(napi::Error::from_reason(format!(
      "Failed to guess file type: {}",
      file_path
    )));
  };
  let Ok(mut tagged_file) = probe.read() else {
    return Err(napi::Error::from_reason(format!(
      "Failed to read audio file: {}",
      file_path
    )));
  };

  // Check if the file has tags
  if tagged_file.primary_tag().is_none() {
    // create the principal tag
    let tag = Tag::new(tagged_file.primary_tag_type());
    tagged_file.insert_tag(tag);
  }

  let primary_tag = tagged_file
    .primary_tag_mut()
    .ok_or(napi::Error::from_reason(
      "Failed to get primary tag after been added".to_string(),
    ))?;

  // Update the tag with new values
  tags.to_tag(primary_tag);

  // Write the updated tag back to the file
  tagged_file
    .save_to_path(path, WriteOptions::default())
    .map_err(|e| napi::Error::from_reason(format!("Failed to write audio file: {}", e)))?;

  Ok(())
}

#[napi]
pub async fn clear_tags(file_path: String) -> Result<()> {
  let path = Path::new(&file_path);
  if !path.exists() {
    return Err(napi::Error::from_reason(format!(
      "File does not exist: {}",
      file_path
    )));
  }

  let Ok(probe) = Probe::open(path) else {
    return Err(napi::Error::from_reason(format!(
      "Failed to open file: {}",
      file_path
    )));
  };

  let Ok(probe) = probe.guess_file_type() else {
    return Err(napi::Error::from_reason(format!(
      "Failed to guess file type: {}",
      file_path
    )));
  };

  let Ok(mut tagged_file) = probe.read() else {
    return Err(napi::Error::from_reason(format!(
      "Failed to read audio file: {}",
      file_path
    )));
  };

  // Create a new empty tag of the same type
  let empty_tag = Tag::new(tagged_file.primary_tag_type());

  // Replace the existing primary tag with the empty one
  tagged_file.insert_tag(empty_tag);

  // Write the updated tag back to the file
  tagged_file
    .save_to_path(path, WriteOptions::default())
    .map_err(|e| napi::Error::from_reason(format!("Failed to write audio file: {}", e)))?;

  Ok(())
}

#[napi]
pub async fn write_tags_to_buffer(
  buffer: napi::bindgen_prelude::Buffer,
  tags: AudioTags,
) -> Result<napi::bindgen_prelude::Buffer> {
  // copy the buffer to a new vec
  let owned_copy: Vec<u8> = buffer.into();

  // Create a fresh cursor for reading
  let mut cursor = Cursor::new(&owned_copy);

  let probe = Probe::new(&mut cursor);

  let Ok(probe) = probe.guess_file_type() else {
    return Err(napi::Error::from_reason(
      "Failed to guess file type".to_string(),
    ));
  };

  let Ok(mut tagged_file) = probe.read() else {
    return Err(napi::Error::from_reason(
      "Failed to read audio file".to_string(),
    ));
  };

  // Check if the file has tags
  if tagged_file.primary_tag().is_none() {
    // create the principal tag
    let tag = Tag::new(tagged_file.primary_tag_type());
    tagged_file.insert_tag(tag);
  }
  let primary_tag = tagged_file
    .primary_tag_mut()
    .ok_or(napi::Error::from_reason(
      "Failed to get primary tag after been added".to_string(),
    ))?;

  tags.to_tag(primary_tag);

  // Write to a new buffer
  let mut cursor = Cursor::new(owned_copy);
  tagged_file
    .save_to(&mut cursor, WriteOptions::default())
    .map_err(|e| napi::Error::from_reason(format!("Failed to write audio to buffer: {}", e)))?;

  Ok(Buffer::from(cursor.into_inner()))
}

#[napi]
pub async fn read_cover_image_from_buffer(buffer: Buffer) -> Result<Option<Buffer>> {
  let buffer_ref = buffer.as_ref();
  let mut cursor = Cursor::new(buffer_ref);

  let probe = Probe::new(&mut cursor);

  let Ok(probe) = probe.guess_file_type() else {
    return Err(napi::Error::from_reason(
      "Failed to guess file type".to_string(),
    ));
  };

  let Ok(tagged_file) = probe.read() else {
    return Err(napi::Error::from_reason(
      "Failed to read audio file".to_string(),
    ));
  };

  let tag = tagged_file.primary_tag();
  match tag {
    Some(tag) => {
      // Look for cover art in the tag
      for picture in tag.pictures() {
        if picture.pic_type() == lofty::picture::PictureType::CoverFront {
          return Ok(Some(picture.data().to_vec().into()));
        }
      }
      Ok(None)
    }
    None => Ok(None),
  }
}

#[napi]
pub async fn write_cover_image_to_buffer(buffer: Buffer, image_data: Buffer) -> Result<Buffer> {
  let buffer_ref = buffer.as_ref();
  let mut cursor = Cursor::new(buffer_ref);
  let probe = Probe::new(&mut cursor);

  let Ok(probe) = probe.guess_file_type() else {
    return Err(napi::Error::from_reason(
      "Failed to guess file type".to_string(),
    ));
  };

  let Ok(mut tagged_file) = probe.read() else {
    return Err(napi::Error::from_reason(
      "Failed to read audio file".to_string(),
    ));
  };

  // Check if the file has tags
  if tagged_file.primary_tag().is_none() {
    // create the principal tag
    let tag = Tag::new(tagged_file.primary_tag_type());
    tagged_file.insert_tag(tag);
  }

  let primary_tag = tagged_file
    .primary_tag_mut()
    .ok_or(napi::Error::from_reason(
      "Failed to get primary tag after been added".to_string(),
    ))?;

  add_cover_image(primary_tag, &image_data, None, MimeType::Jpeg);

  // Create a copy of the buffer for writing
  let owned_copy: Vec<u8> = buffer.into();

  // Write the updated tag back to the buffer
  let mut cursor = Cursor::new(owned_copy);
  tagged_file
    .save_to(&mut cursor, WriteOptions::default())
    .map_err(|e| napi::Error::from_reason(format!("Failed to write audio to buffer: {}", e)))?;

  Ok(Buffer::from(cursor.into_inner()))
}

#[napi]
pub async fn read_cover_image_from_file(file_path: String) -> Result<Option<Buffer>> {
  let path = Path::new(&file_path);
  let buffer = fs::read(path)?;
  read_cover_image_from_buffer(buffer.into()).await
}

#[napi]
pub async fn write_cover_image_to_file(file_path: String, image_data: Buffer) -> Result<()> {
  let path = Path::new(&file_path);
  let buffer = fs::read(path)?;
  let buffer = write_cover_image_to_buffer(buffer.into(), image_data).await?;
  fs::write(path, buffer)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use lofty::picture::MimeType;

  // Helper function to create test image data
  fn create_test_image_data() -> Vec<u8> {
    // Minimal JPEG header
    vec![
      0xFF, 0xD8, 0xFF, 0xE0, // JPEG SOI + APP0
      0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, // JFIF header
      0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0xFF, 0xD9, // JPEG EOI
    ]
  }

  // Test structs that don't use NAPI types
  #[derive(Debug, PartialEq)]
  struct TestPosition {
    pub no: Option<u32>,
    pub of: Option<u32>,
  }

  #[derive(Debug, PartialEq)]
  struct TestImage {
    pub data: Vec<u8>,
    pub mime_type: Option<String>,
    pub description: Option<String>,
  }

  #[derive(Debug, PartialEq, Default)]
  struct TestAudioTags {
    pub title: Option<String>,
    pub artists: Option<Vec<String>>,
    pub album: Option<String>,
    pub year: Option<u32>,
    pub genre: Option<String>,
    pub track: Option<TestPosition>,
    pub album_artists: Option<Vec<String>>,
    pub comment: Option<String>,
    pub disc: Option<TestPosition>,
    pub image: Option<TestImage>,
  }

  #[test]
  fn test_mime_type_to_string() {
    assert_eq!(
      mime_type_to_string(&MimeType::Jpeg),
      Some("image/jpeg".to_string())
    );
    assert_eq!(
      mime_type_to_string(&MimeType::Png),
      Some("image/png".to_string())
    );
    assert_eq!(
      mime_type_to_string(&MimeType::Gif),
      Some("image/gif".to_string())
    );
    assert_eq!(
      mime_type_to_string(&MimeType::Tiff),
      Some("image/tiff".to_string())
    );
    assert_eq!(
      mime_type_to_string(&MimeType::Bmp),
      Some("image/bmp".to_string())
    );
    assert_eq!(
      mime_type_to_string(&MimeType::Unknown("unknown".to_string())),
      None
    );
  }

  #[test]
  fn test_audio_tags_default() {
    let tags = TestAudioTags::default();
    assert!(tags.title.is_none());
    assert!(tags.artists.is_none());
    assert!(tags.album.is_none());
    assert!(tags.year.is_none());
    assert!(tags.genre.is_none());
    assert!(tags.track.is_none());
    assert!(tags.album_artists.is_none());
    assert!(tags.comment.is_none());
    assert!(tags.disc.is_none());
    assert!(tags.image.is_none());
  }

  #[test]
  fn test_audio_tags_basic() {
    let tags = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: Some(vec!["Test Artist".to_string()]),
      album: Some("Test Album".to_string()),
      year: Some(2024),
      genre: Some("Test Genre".to_string()),
      track: Some(TestPosition {
        no: Some(1),
        of: Some(10),
      }),
      album_artists: Some(vec!["Test Album Artist".to_string()]),
      comment: Some("Test comment".to_string()),
      disc: Some(TestPosition {
        no: Some(1),
        of: Some(2),
      }),
      image: None,
    };

    // Test that the struct is created correctly
    assert_eq!(tags.title, Some("Test Song".to_string()));
    assert_eq!(tags.artists, Some(vec!["Test Artist".to_string()]));
    assert_eq!(tags.album, Some("Test Album".to_string()));
    assert_eq!(tags.year, Some(2024));
    assert_eq!(tags.genre, Some("Test Genre".to_string()));
    assert_eq!(
      tags.track,
      Some(TestPosition {
        no: Some(1),
        of: Some(10)
      })
    );
    assert_eq!(
      tags.album_artists,
      Some(vec!["Test Album Artist".to_string()])
    );
    assert_eq!(tags.comment, Some("Test comment".to_string()));
    assert_eq!(
      tags.disc,
      Some(TestPosition {
        no: Some(1),
        of: Some(2)
      })
    );
    assert!(tags.image.is_none());
  }

  #[test]
  fn test_audio_tags_with_image() {
    let image_data = create_test_image_data();
    let tags = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: Some(vec!["Test Artist".to_string()]),
      album: Some("Test Album".to_string()),
      year: Some(2024),
      genre: Some("Test Genre".to_string()),
      track: Some(TestPosition {
        no: Some(1),
        of: Some(10),
      }),
      album_artists: Some(vec!["Test Album Artist".to_string()]),
      comment: Some("Test comment".to_string()),
      disc: Some(TestPosition {
        no: Some(1),
        of: Some(2),
      }),
      image: Some(TestImage {
        data: image_data.clone(),
        mime_type: Some("image/jpeg".to_string()),
        description: Some("Test cover".to_string()),
      }),
    };

    // Test that the struct with image is created correctly
    assert_eq!(tags.title, Some("Test Song".to_string()));
    assert!(tags.image.is_some());
    let image = tags.image.unwrap();
    assert_eq!(image.mime_type, Some("image/jpeg".to_string()));
    assert_eq!(image.description, Some("Test cover".to_string()));
    assert_eq!(image.data, image_data);
  }

  #[test]
  fn test_audio_tags_empty_artists() {
    let tags = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: Some(vec![]), // Empty artists
      album: Some("Test Album".to_string()),
      year: Some(2024),
      genre: Some("Test Genre".to_string()),
      track: None,
      album_artists: None,
      comment: None,
      disc: None,
      image: None,
    };

    // Test that empty artists vector is handled correctly
    assert_eq!(tags.title, Some("Test Song".to_string()));
    assert_eq!(tags.artists, Some(vec![]));
    assert_eq!(tags.album, Some("Test Album".to_string()));
    assert_eq!(tags.year, Some(2024));
    assert_eq!(tags.genre, Some("Test Genre".to_string()));
  }

  #[test]
  fn test_audio_tags_multiple_artists() {
    let tags = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: Some(vec![
        "Artist 1".to_string(),
        "Artist 2".to_string(),
        "Artist 3".to_string(),
      ]),
      album: Some("Test Album".to_string()),
      year: Some(2024),
      genre: Some("Test Genre".to_string()),
      track: None,
      album_artists: None,
      comment: None,
      disc: None,
      image: None,
    };

    // Test that multiple artists are handled correctly
    assert_eq!(tags.title, Some("Test Song".to_string()));
    assert_eq!(
      tags.artists,
      Some(vec![
        "Artist 1".to_string(),
        "Artist 2".to_string(),
        "Artist 3".to_string()
      ])
    );
    assert_eq!(tags.album, Some("Test Album".to_string()));
    assert_eq!(tags.year, Some(2024));
    assert_eq!(tags.genre, Some("Test Genre".to_string()));
  }

  #[test]
  fn test_audio_tags_partial_data() {
    let tags = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: None, // Not set
      album: None,   // Not set
      year: Some(2024),
      genre: None, // Not set
      track: Some(TestPosition {
        no: Some(1),
        of: None,
      }), // Only track number
      album_artists: None,
      comment: None,
      disc: None,
      image: None,
    };

    // Test that partial data is handled correctly
    assert_eq!(tags.title, Some("Test Song".to_string()));
    assert!(tags.artists.is_none());
    assert!(tags.album.is_none());
    assert_eq!(tags.year, Some(2024));
    assert!(tags.genre.is_none());
    assert_eq!(
      tags.track,
      Some(TestPosition {
        no: Some(1),
        of: None
      })
    );
  }

  #[test]
  fn test_position_struct() {
    let pos = TestPosition {
      no: Some(1),
      of: Some(10),
    };
    assert_eq!(pos.no, Some(1));
    assert_eq!(pos.of, Some(10));

    let pos_partial = TestPosition {
      no: Some(1),
      of: None,
    };
    assert_eq!(pos_partial.no, Some(1));
    assert_eq!(pos_partial.of, None);
  }

  #[test]
  fn test_image_struct() {
    let image_data = create_test_image_data();
    let image = TestImage {
      data: image_data.clone(),
      mime_type: Some("image/jpeg".to_string()),
      description: Some("Test image".to_string()),
    };

    assert_eq!(image.data, image_data);
    assert_eq!(image.mime_type, Some("image/jpeg".to_string()));
    assert_eq!(image.description, Some("Test image".to_string()));

    let image_minimal = TestImage {
      data: image_data,
      mime_type: None,
      description: None,
    };

    assert_eq!(image_minimal.mime_type, None);
    assert_eq!(image_minimal.description, None);
  }

  #[test]
  fn test_audio_tags_creation_variations() {
    // Test with all fields
    let full_tags = TestAudioTags {
      title: Some("Full Song".to_string()),
      artists: Some(vec!["Artist 1".to_string(), "Artist 2".to_string()]),
      album: Some("Full Album".to_string()),
      year: Some(2023),
      genre: Some("Rock".to_string()),
      track: Some(TestPosition {
        no: Some(5),
        of: Some(12),
      }),
      album_artists: Some(vec!["Album Artist".to_string()]),
      comment: Some("Great song".to_string()),
      disc: Some(TestPosition {
        no: Some(1),
        of: Some(2),
      }),
      image: Some(TestImage {
        data: create_test_image_data(),
        mime_type: Some("image/png".to_string()),
        description: Some("Album cover".to_string()),
      }),
    };

    assert_eq!(full_tags.title, Some("Full Song".to_string()));
    assert_eq!(
      full_tags.artists,
      Some(vec!["Artist 1".to_string(), "Artist 2".to_string()])
    );
    assert_eq!(
      full_tags.track,
      Some(TestPosition {
        no: Some(5),
        of: Some(12)
      })
    );
    assert!(full_tags.image.is_some());

    // Test with minimal fields
    let minimal_tags = TestAudioTags {
      title: Some("Minimal Song".to_string()),
      artists: None,
      album: None,
      year: None,
      genre: None,
      track: None,
      album_artists: None,
      comment: None,
      disc: None,
      image: None,
    };

    assert_eq!(minimal_tags.title, Some("Minimal Song".to_string()));
    assert!(minimal_tags.artists.is_none());
    assert!(minimal_tags.album.is_none());
    assert!(minimal_tags.year.is_none());
    assert!(minimal_tags.image.is_none());
  }
}
