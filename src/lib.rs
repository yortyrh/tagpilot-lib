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

  // Additional comprehensive tests for better coverage

  #[test]
  fn test_mime_type_to_string_edge_cases() {
    // Test all supported MIME types
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

    // Test unsupported MIME types
    assert_eq!(
      mime_type_to_string(&MimeType::Unknown("unsupported".to_string())),
      None
    );
  }

  #[test]
  fn test_position_struct_edge_cases() {
    // Test with both values
    let pos_full = TestPosition {
      no: Some(1),
      of: Some(10),
    };
    assert_eq!(pos_full.no, Some(1));
    assert_eq!(pos_full.of, Some(10));

    // Test with only no
    let pos_no_only = TestPosition {
      no: Some(5),
      of: None,
    };
    assert_eq!(pos_no_only.no, Some(5));
    assert_eq!(pos_no_only.of, None);

    // Test with only of
    let pos_of_only = TestPosition {
      no: None,
      of: Some(15),
    };
    assert_eq!(pos_of_only.no, None);
    assert_eq!(pos_of_only.of, Some(15));

    // Test with neither
    let pos_empty = TestPosition {
      no: None,
      of: None,
    };
    assert_eq!(pos_empty.no, None);
    assert_eq!(pos_empty.of, None);

    // Test with zero values
    let pos_zero = TestPosition {
      no: Some(0),
      of: Some(0),
    };
    assert_eq!(pos_zero.no, Some(0));
    assert_eq!(pos_zero.of, Some(0));

    // Test with large values
    let pos_large = TestPosition {
      no: Some(999),
      of: Some(1000),
    };
    assert_eq!(pos_large.no, Some(999));
    assert_eq!(pos_large.of, Some(1000));
  }

  #[test]
  fn test_image_struct_edge_cases() {
    let image_data = create_test_image_data();

    // Test with all fields
    let image_full = TestImage {
      data: image_data.clone(),
      mime_type: Some("image/jpeg".to_string()),
      description: Some("Full description".to_string()),
    };
    assert_eq!(image_full.data, image_data);
    assert_eq!(image_full.mime_type, Some("image/jpeg".to_string()));
    assert_eq!(image_full.description, Some("Full description".to_string()));

    // Test with no optional fields
    let image_minimal = TestImage {
      data: image_data.clone(),
      mime_type: None,
      description: None,
    };
    assert_eq!(image_minimal.data, image_data);
    assert_eq!(image_minimal.mime_type, None);
    assert_eq!(image_minimal.description, None);

    // Test with only mime_type
    let image_mime_only = TestImage {
      data: image_data.clone(),
      mime_type: Some("image/png".to_string()),
      description: None,
    };
    assert_eq!(image_mime_only.mime_type, Some("image/png".to_string()));
    assert_eq!(image_mime_only.description, None);

    // Test with only description
    let image_desc_only = TestImage {
      data: image_data.clone(),
      mime_type: None,
      description: Some("Description only".to_string()),
    };
    assert_eq!(image_desc_only.mime_type, None);
    assert_eq!(image_desc_only.description, Some("Description only".to_string()));

    // Test with empty data
    let image_empty = TestImage {
      data: vec![],
      mime_type: Some("image/jpeg".to_string()),
      description: Some("Empty data".to_string()),
    };
    assert_eq!(image_empty.data, vec![]);
    assert_eq!(image_empty.mime_type, Some("image/jpeg".to_string()));
    assert_eq!(image_empty.description, Some("Empty data".to_string()));

    // Test with empty strings
    let image_empty_strings = TestImage {
      data: image_data,
      mime_type: Some("".to_string()),
      description: Some("".to_string()),
    };
    assert_eq!(image_empty_strings.mime_type, Some("".to_string()));
    assert_eq!(image_empty_strings.description, Some("".to_string()));
  }

  #[test]
  fn test_audio_tags_string_edge_cases() {
    // Test with empty strings
    let tags_empty_strings = TestAudioTags {
      title: Some("".to_string()),
      artists: Some(vec!["".to_string()]),
      album: Some("".to_string()),
      year: Some(2024),
      genre: Some("".to_string()),
      track: None,
      album_artists: Some(vec!["".to_string()]),
      comment: Some("".to_string()),
      disc: None,
      image: None,
    };

    assert_eq!(tags_empty_strings.title, Some("".to_string()));
    assert_eq!(tags_empty_strings.artists, Some(vec!["".to_string()]));
    assert_eq!(tags_empty_strings.album, Some("".to_string()));
    assert_eq!(tags_empty_strings.genre, Some("".to_string()));
    assert_eq!(tags_empty_strings.album_artists, Some(vec!["".to_string()]));
    assert_eq!(tags_empty_strings.comment, Some("".to_string()));

    // Test with very long strings
    let long_string = "a".repeat(1000);
    let tags_long_strings = TestAudioTags {
      title: Some(long_string.clone()),
      artists: Some(vec![long_string.clone()]),
      album: Some(long_string.clone()),
      year: Some(2024),
      genre: Some(long_string.clone()),
      track: None,
      album_artists: Some(vec![long_string.clone()]),
      comment: Some(long_string.clone()),
      disc: None,
      image: None,
    };

    assert_eq!(tags_long_strings.title, Some(long_string.clone()));
    assert_eq!(tags_long_strings.artists, Some(vec![long_string.clone()]));
    assert_eq!(tags_long_strings.album, Some(long_string.clone()));
    assert_eq!(tags_long_strings.genre, Some(long_string.clone()));
    assert_eq!(tags_long_strings.album_artists, Some(vec![long_string.clone()]));
    assert_eq!(tags_long_strings.comment, Some(long_string));

    // Test with special characters
    let special_chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?`~";
    let tags_special = TestAudioTags {
      title: Some(special_chars.to_string()),
      artists: Some(vec![special_chars.to_string()]),
      album: Some(special_chars.to_string()),
      year: Some(2024),
      genre: Some(special_chars.to_string()),
      track: None,
      album_artists: Some(vec![special_chars.to_string()]),
      comment: Some(special_chars.to_string()),
      disc: None,
      image: None,
    };

    assert_eq!(tags_special.title, Some(special_chars.to_string()));
    assert_eq!(tags_special.artists, Some(vec![special_chars.to_string()]));
    assert_eq!(tags_special.album, Some(special_chars.to_string()));
    assert_eq!(tags_special.genre, Some(special_chars.to_string()));
    assert_eq!(tags_special.album_artists, Some(vec![special_chars.to_string()]));
    assert_eq!(tags_special.comment, Some(special_chars.to_string()));

    // Test with unicode characters
    let unicode_string = "🎵 音乐 🎶 音楽 🎼";
    let tags_unicode = TestAudioTags {
      title: Some(unicode_string.to_string()),
      artists: Some(vec![unicode_string.to_string()]),
      album: Some(unicode_string.to_string()),
      year: Some(2024),
      genre: Some(unicode_string.to_string()),
      track: None,
      album_artists: Some(vec![unicode_string.to_string()]),
      comment: Some(unicode_string.to_string()),
      disc: None,
      image: None,
    };

    assert_eq!(tags_unicode.title, Some(unicode_string.to_string()));
    assert_eq!(tags_unicode.artists, Some(vec![unicode_string.to_string()]));
    assert_eq!(tags_unicode.album, Some(unicode_string.to_string()));
    assert_eq!(tags_unicode.genre, Some(unicode_string.to_string()));
    assert_eq!(tags_unicode.album_artists, Some(vec![unicode_string.to_string()]));
    assert_eq!(tags_unicode.comment, Some(unicode_string.to_string()));
  }

  #[test]
  fn test_audio_tags_year_edge_cases() {
    // Test with various years
    let years = vec![1900, 1950, 2000, 2024, 2030, 9999];
    
    for year in years {
      let tags = TestAudioTags {
        title: Some("Test Song".to_string()),
        artists: None,
        album: None,
        year: Some(year),
        genre: None,
        track: None,
        album_artists: None,
        comment: None,
        disc: None,
        image: None,
      };
      assert_eq!(tags.year, Some(year));
    }

    // Test with year 0 (edge case)
    let tags_year_zero = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: None,
      album: None,
      year: Some(0),
      genre: None,
      track: None,
      album_artists: None,
      comment: None,
      disc: None,
      image: None,
    };
    assert_eq!(tags_year_zero.year, Some(0));
  }

  #[test]
  fn test_audio_tags_artists_edge_cases() {
    // Test with single artist
    let tags_single = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: Some(vec!["Single Artist".to_string()]),
      album: None,
      year: None,
      genre: None,
      track: None,
      album_artists: None,
      comment: None,
      disc: None,
      image: None,
    };
    assert_eq!(tags_single.artists, Some(vec!["Single Artist".to_string()]));

    // Test with many artists
    let many_artists: Vec<String> = (1..=50)
      .map(|i| format!("Artist {}", i))
      .collect();
    let tags_many = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: Some(many_artists.clone()),
      album: None,
      year: None,
      genre: None,
      track: None,
      album_artists: None,
      comment: None,
      disc: None,
      image: None,
    };
    assert_eq!(tags_many.artists, Some(many_artists));

    // Test with duplicate artists
    let tags_duplicates = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: Some(vec![
        "Same Artist".to_string(),
        "Same Artist".to_string(),
        "Same Artist".to_string(),
      ]),
      album: None,
      year: None,
      genre: None,
      track: None,
      album_artists: None,
      comment: None,
      disc: None,
      image: None,
    };
    assert_eq!(
      tags_duplicates.artists,
      Some(vec![
        "Same Artist".to_string(),
        "Same Artist".to_string(),
        "Same Artist".to_string(),
      ])
    );
  }

  #[test]
  fn test_audio_tags_track_disc_edge_cases() {
    // Test track with zero values
    let tags_track_zero = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: None,
      album: None,
      year: None,
      genre: None,
      track: Some(TestPosition {
        no: Some(0),
        of: Some(0),
      }),
      album_artists: None,
      comment: None,
      disc: Some(TestPosition {
        no: Some(0),
        of: Some(0),
      }),
      image: None,
    };
    assert_eq!(
      tags_track_zero.track,
      Some(TestPosition {
        no: Some(0),
        of: Some(0)
      })
    );
    assert_eq!(
      tags_track_zero.disc,
      Some(TestPosition {
        no: Some(0),
        of: Some(0)
      })
    );

    // Test track with large values
    let tags_track_large = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: None,
      album: None,
      year: None,
      genre: None,
      track: Some(TestPosition {
        no: Some(999),
        of: Some(1000),
      }),
      album_artists: None,
      comment: None,
      disc: Some(TestPosition {
        no: Some(99),
        of: Some(100),
      }),
      image: None,
    };
    assert_eq!(
      tags_track_large.track,
      Some(TestPosition {
        no: Some(999),
        of: Some(1000)
      })
    );
    assert_eq!(
      tags_track_large.disc,
      Some(TestPosition {
        no: Some(99),
        of: Some(100)
      })
    );

    // Test track where no > of (invalid but should be handled)
    let tags_track_invalid = TestAudioTags {
      title: Some("Test Song".to_string()),
      artists: None,
      album: None,
      year: None,
      genre: None,
      track: Some(TestPosition {
        no: Some(10),
        of: Some(5), // no > of
      }),
      album_artists: None,
      comment: None,
      disc: Some(TestPosition {
        no: Some(3),
        of: Some(1), // no > of
      }),
      image: None,
    };
    assert_eq!(
      tags_track_invalid.track,
      Some(TestPosition {
        no: Some(10),
        of: Some(5)
      })
    );
    assert_eq!(
      tags_track_invalid.disc,
      Some(TestPosition {
        no: Some(3),
        of: Some(1)
      })
    );
  }

  #[test]
  fn test_audio_tags_combination_scenarios() {
    // Test realistic music metadata scenarios
    let classical_tags = TestAudioTags {
      title: Some("Symphony No. 9 in D minor, Op. 125".to_string()),
      artists: Some(vec!["Ludwig van Beethoven".to_string()]),
      album: Some("Beethoven: Complete Symphonies".to_string()),
      year: Some(1824),
      genre: Some("Classical".to_string()),
      track: Some(TestPosition {
        no: Some(1),
        of: Some(4),
      }),
      album_artists: Some(vec!["Berlin Philharmonic".to_string()]),
      comment: Some("Conducted by Herbert von Karajan".to_string()),
      disc: Some(TestPosition {
        no: Some(1),
        of: Some(5),
      }),
      image: Some(TestImage {
        data: create_test_image_data(),
        mime_type: Some("image/jpeg".to_string()),
        description: Some("Album cover art".to_string()),
      }),
    };

    assert_eq!(
      classical_tags.title,
      Some("Symphony No. 9 in D minor, Op. 125".to_string())
    );
    assert_eq!(
      classical_tags.artists,
      Some(vec!["Ludwig van Beethoven".to_string()])
    );
    assert_eq!(classical_tags.year, Some(1824));
    assert_eq!(classical_tags.genre, Some("Classical".to_string()));

    // Test modern pop song scenario
    let pop_tags = TestAudioTags {
      title: Some("Shape of You".to_string()),
      artists: Some(vec!["Ed Sheeran".to_string()]),
      album: Some("÷ (Divide)".to_string()),
      year: Some(2017),
      genre: Some("Pop".to_string()),
      track: Some(TestPosition {
        no: Some(3),
        of: Some(16),
      }),
      album_artists: Some(vec!["Ed Sheeran".to_string()]),
      comment: Some("Produced by Steve Mac".to_string()),
      disc: None,
      image: None,
    };

    assert_eq!(pop_tags.title, Some("Shape of You".to_string()));
    assert_eq!(pop_tags.artists, Some(vec!["Ed Sheeran".to_string()]));
    assert_eq!(pop_tags.year, Some(2017));
    assert_eq!(pop_tags.genre, Some("Pop".to_string()));

    // Test compilation album scenario
    let compilation_tags = TestAudioTags {
      title: Some("Bohemian Rhapsody".to_string()),
      artists: Some(vec!["Queen".to_string()]),
      album: Some("Greatest Hits".to_string()),
      year: Some(1975),
      genre: Some("Rock".to_string()),
      track: Some(TestPosition {
        no: Some(1),
        of: Some(17),
      }),
      album_artists: Some(vec!["Various Artists".to_string()]),
      comment: Some("From the album 'A Night at the Opera'".to_string()),
      disc: Some(TestPosition {
        no: Some(1),
        of: Some(2),
      }),
      image: Some(TestImage {
        data: create_test_image_data(),
        mime_type: Some("image/png".to_string()),
        description: Some("Compilation cover".to_string()),
      }),
    };

    assert_eq!(compilation_tags.title, Some("Bohemian Rhapsody".to_string()));
    assert_eq!(compilation_tags.artists, Some(vec!["Queen".to_string()]));
    assert_eq!(compilation_tags.album_artists, Some(vec!["Various Artists".to_string()]));
    assert_eq!(compilation_tags.year, Some(1975));
  }

  #[test]
  fn test_create_test_image_data() {
    let image_data = create_test_image_data();
    
    // Test that the image data is not empty
    assert!(!image_data.is_empty());
    
    // Test JPEG header structure
    assert_eq!(image_data[0], 0xFF); // JPEG SOI marker
    assert_eq!(image_data[1], 0xD8); // JPEG SOI marker
    assert_eq!(image_data[2], 0xFF); // APP0 marker
    assert_eq!(image_data[3], 0xE0); // APP0 marker
    
    // Test JFIF identifier
    assert_eq!(image_data[6], 0x4A); // 'J'
    assert_eq!(image_data[7], 0x46); // 'F'
    assert_eq!(image_data[8], 0x49); // 'I'
    assert_eq!(image_data[9], 0x46); // 'F'
    
    // Test JPEG EOI marker
    let last_two = &image_data[image_data.len() - 2..];
    assert_eq!(last_two[0], 0xFF); // JPEG EOI marker
    assert_eq!(last_two[1], 0xD9); // JPEG EOI marker
    
    // Test that multiple calls return the same data
    let image_data2 = create_test_image_data();
    assert_eq!(image_data, image_data2);
  }
}
