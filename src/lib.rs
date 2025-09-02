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
use std::io::{Cursor, Read};
use std::path::Path;

#[napi(object)]
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

fn add_cover_image(primary_tag: &mut Tag, image_data: &Buffer, default_mime_type: MimeType) {
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
  let picture = Picture::new_unchecked(
    lofty::picture::PictureType::CoverFront,
    Some(mime_type),
    None,
    buf,
  );
  primary_tag.push_picture(picture);
}

impl Default for AudioTags {
  fn default() -> Self {
    Self {
      title: None,
      artists: None,
      album: None,
      year: None,
      genre: None,
      track: None,
      album_artists: None,
      comment: None,
      disc: None,
      image: None,
    }
  }
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
        _ => Some(Position {
          no: tag.track(),
          of: tag.track_total(),
        }),
      },
      album_artists: tag.artist().map(|s| vec![s.to_string()]),
      comment: tag.comment().map(|s| s.to_string()),
      disc: match (tag.disk(), tag.disk_total()) {
        (None, None) => None,
        _ => Some(Position {
          no: tag.disk(),
          of: tag.disk_total(),
        }),
      },
      image: {
        let mut image = None;
        for picture in tag.pictures() {
          if picture.pic_type() == lofty::picture::PictureType::CoverFront {
            image = Some(Image {
              data: picture.data().to_vec().into(),
              mime_type: mime_type_to_string(&picture.mime_type().unwrap()),
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
    if let Some(title) = &self.title {
      primary_tag.insert_text(ItemKey::TrackTitle, title.clone());
    }
    if let Some(artists) = &self.artists {
      if let Some(artist) = artists.first() {
        primary_tag.insert_text(ItemKey::TrackArtist, artist.clone());
      }
    }
    if let Some(album) = &self.album {
      primary_tag.insert_text(ItemKey::AlbumTitle, album.clone());
    }
    if let Some(year) = &self.year {
      primary_tag.insert_text(ItemKey::Year, year.to_string());
    }
    if let Some(genre) = &self.genre {
      primary_tag.insert_text(ItemKey::Genre, genre.clone());
    }
    if let Some(track_info) = &self.track {
      if let Some(track_no) = track_info.no {
        primary_tag.insert_text(ItemKey::TrackNumber, track_no.to_string());
      }
      if let Some(track_total) = track_info.of {
        primary_tag.insert_text(ItemKey::TrackTotal, track_total.to_string());
      }
    }
    if let Some(album_artists) = &self.album_artists {
      if let Some(album_artist) = album_artists.first() {
        primary_tag.insert_text(ItemKey::AlbumArtist, album_artist.clone());
      }
    }
    if let Some(comment) = &self.comment {
      primary_tag.insert_text(ItemKey::Comment, comment.clone());
    }
    if let Some(disc_info) = &self.disc {
      if let Some(disc_no) = disc_info.no {
        primary_tag.insert_text(ItemKey::DiscNumber, disc_no.to_string());
      }
      if let Some(disc_total) = disc_info.of {
        primary_tag.insert_text(ItemKey::DiscTotal, disc_total.to_string());
      }
    }

    if let Some(image) = &self.image {
      add_cover_image(
        primary_tag,
        &image.data,
        image
          .mime_type
          .as_ref()
          .map(|s| MimeType::from_str(&s))
          .unwrap_or(MimeType::Jpeg),
      );
    }
  }
}

#[napi]
pub async fn read_tags(file_path: String) -> Result<AudioTags> {
  let path = Path::new(&file_path);

  match Probe::open(path).unwrap().guess_file_type().unwrap().read() {
    Ok(tagged_file) => {
      let tag = tagged_file.primary_tag();
      match tag {
        Some(tag) => Ok(AudioTags::from_tag(tag)),
        None => Ok(AudioTags::default()),
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
        Some(tag) => Ok(AudioTags::from_tag(tag)),
        None => Ok(AudioTags::default()),
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
  tags.to_tag(primary_tag);

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
pub async fn write_tags_to_buffer(
  buffer: napi::bindgen_prelude::Buffer,
  tags: AudioTags,
) -> Result<napi::bindgen_prelude::Buffer> {
  // copy the buffer to a new vec
  let owned_copy: Vec<u8> = buffer.to_vec();

  // Create a fresh cursor for reading
  let mut input_cursor = Cursor::new(&owned_copy);

  // Read the existing file from buffer
  let mut tagged_file = match Probe::new(input_cursor.by_ref())
    .guess_file_type()
    .unwrap()
    .read()
  {
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

  tags.to_tag(primary_tag);

  // Write to a new buffer
  let mut cursor = Cursor::new(owned_copy);
  match tagged_file.save_to(&mut cursor, WriteOptions::default()) {
    Ok(_) => {
      let owned_copy_buffer = Buffer::from(cursor.into_inner());
      Ok(owned_copy_buffer)
    }
    Err(e) => Err(napi::Error::from_reason(format!(
      "Failed to write audio to buffer: {}",
      e
    ))),
  }
}

#[napi]
pub async fn read_cover_image_from_buffer(buffer: Buffer) -> Result<Option<Buffer>> {
  let buffer_ref = buffer.as_ref();
  let mut cursor = Cursor::new(buffer_ref);

  match Probe::new(&mut cursor).guess_file_type().unwrap().read() {
    Ok(tagged_file) => {
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
    Err(e) => Err(napi::Error::from_reason(format!(
      "Failed to read audio from buffer: {}",
      e
    ))),
  }
}

#[napi]
pub async fn write_cover_image_to_buffer(buffer: Buffer, image_data: Buffer) -> Result<Buffer> {
  let buffer_ref = buffer.as_ref();
  let mut cursor = Cursor::new(buffer_ref);

  // Read the existing file from buffer
  let mut tagged_file = match Probe::new(&mut cursor).guess_file_type().unwrap().read() {
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

  add_cover_image(primary_tag, &image_data, MimeType::Jpeg);

  // Create a copy of the buffer for writing
  let owned_copy: Vec<u8> = buffer.to_vec();
  let mut output_cursor = Cursor::new(owned_copy);

  // Write the updated tag back to the buffer
  match tagged_file.save_to(&mut output_cursor, WriteOptions::default()) {
    Ok(_) => {
      let modified_buffer = Buffer::from(output_cursor.into_inner());
      Ok(modified_buffer)
    }
    Err(e) => Err(napi::Error::from_reason(format!(
      "Failed to write audio to buffer: {}",
      e
    ))),
  }
}

#[napi]
pub async fn read_cover_image_from_file(file_path: String) -> Result<Option<Buffer>> {
  let path = Path::new(&file_path);
  let buffer = fs::read(path).unwrap();
  read_cover_image_from_buffer(buffer.into()).await
}

#[napi]
pub async fn write_cover_image_to_file(file_path: String, image_data: Buffer) -> Result<()> {
  let path = Path::new(&file_path);
  let buffer = fs::read(path).unwrap();
  let buffer = write_cover_image_to_buffer(buffer.into(), image_data).await?;
  fs::write(path, buffer).unwrap();
  Ok(())
}
