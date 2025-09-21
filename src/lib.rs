#![deny(clippy::all)]

mod util;

use crate::util::{AudioTags, Image, Position};
use napi::bindgen_prelude::Buffer;
use napi::Result;
use napi_derive::napi;

#[napi(js_name = "Position", object)]
#[derive(Debug, PartialEq)]
pub struct ApiPosition {
  pub no: Option<u32>,
  pub of: Option<u32>,
}

impl ApiPosition {
  pub fn from_position(position: Position) -> Self {
    Self {
      no: position.no,
      of: position.of,
    }
  }

  pub fn into_position(self) -> Position {
    Position {
      no: self.no,
      of: self.of,
    }
  }
}

#[napi(js_name = "Image", object)]
pub struct ApiImage {
  pub data: Buffer,
  pub mime_type: Option<String>,
  pub description: Option<String>,
}

impl ApiImage {
  pub fn from_image(image: Image) -> Self {
    Self {
      data: Buffer::from(image.data),
      mime_type: image.mime_type,
      description: image.description,
    }
  }

  pub fn into_image(self) -> Image {
    Image {
      data: self.data.to_vec(),
      mime_type: self.mime_type,
      description: self.description,
    }
  }
}

#[napi(js_name = "AudioTags", object)]
#[derive(Default)]
pub struct ApiAudioTags {
  pub title: Option<String>,
  pub artists: Option<Vec<String>>,
  pub album: Option<String>,
  pub year: Option<u32>,
  pub genre: Option<String>,
  pub track: Option<ApiPosition>,
  pub album_artists: Option<Vec<String>>,
  pub comment: Option<String>,
  pub disc: Option<ApiPosition>,
  pub image: Option<ApiImage>,
}

impl ApiAudioTags {
  pub fn from_audio_tags(audio_tags: AudioTags) -> Self {
    Self {
      title: audio_tags.title,
      artists: audio_tags.artists,
      album: audio_tags.album,
      year: audio_tags.year,
      genre: audio_tags.genre,
      track: audio_tags.track.map(ApiPosition::from_position),
      album_artists: audio_tags.album_artists,
      comment: audio_tags.comment,
      disc: audio_tags.disc.map(ApiPosition::from_position),
      image: audio_tags.image.map(ApiImage::from_image),
    }
  }

  pub fn into_audio_tags(self) -> AudioTags {
    AudioTags {
      title: self.title,
      artists: self.artists,
      album: self.album,
      year: self.year,
      genre: self.genre,
      track: self.track.map(|position| position.into_position()),
      album_artists: self.album_artists,
      comment: self.comment,
      disc: self.disc.map(|position| position.into_position()),
      image: self.image.map(|image| image.into_image()),
    }
  }
}

#[napi]
pub async fn read_tags(file_path: String) -> Result<ApiAudioTags> {
  let tags = util::read_tags(file_path)
    .await
    .map_err(napi::Error::from_reason)?;
  Ok(ApiAudioTags::from_audio_tags(tags))
}

#[napi]
pub async fn read_tags_from_buffer(buffer: napi::bindgen_prelude::Buffer) -> Result<ApiAudioTags> {
  let tags = util::read_tags_from_buffer(buffer.to_vec())
    .await
    .map_err(napi::Error::from_reason)?;
  Ok(ApiAudioTags::from_audio_tags(tags))
}

#[napi]
pub async fn write_tags(file_path: String, tags: ApiAudioTags) -> Result<()> {
  util::write_tags(file_path, tags.into_audio_tags())
    .await
    .map_err(napi::Error::from_reason)
}

#[napi]
pub async fn write_tags_to_buffer(
  buffer: napi::bindgen_prelude::Buffer,
  tags: ApiAudioTags,
) -> Result<napi::bindgen_prelude::Buffer> {
  let result = util::write_tags_to_buffer(buffer.to_vec(), tags.into_audio_tags())
    .await
    .map_err(napi::Error::from_reason)?;
  Ok(Buffer::from(result))
}

#[napi]
pub async fn clear_tags(file_path: String) -> Result<()> {
  util::clear_tags(file_path)
    .await
    .map_err(napi::Error::from_reason)
}

#[napi]
pub async fn clear_tags_to_buffer(buffer: Buffer) -> Result<Buffer> {
  let result = util::clear_tags_to_buffer(buffer.to_vec())
    .await
    .map_err(napi::Error::from_reason)?;
  Ok(Buffer::from(result))
}

#[napi]
pub async fn read_cover_image_from_buffer(buffer: Buffer) -> Result<Option<Buffer>> {
  let result = util::read_cover_image_from_buffer(buffer.to_vec())
    .await
    .map_err(napi::Error::from_reason)?;
  Ok(result.map(Buffer::from))
}

#[napi]
pub async fn write_cover_image_to_buffer(buffer: Buffer, image_data: Buffer) -> Result<Buffer> {
  let result = util::write_cover_image_to_buffer(buffer.to_vec(), image_data.to_vec())
    .await
    .map_err(napi::Error::from_reason)?;
  Ok(Buffer::from(result))
}

#[napi]
pub async fn read_cover_image_from_file(file_path: String) -> Result<Option<Buffer>> {
  let result = util::read_cover_image_from_file(file_path)
    .await
    .map_err(napi::Error::from_reason)?;
  Ok(result.map(Buffer::from))
}

#[napi]
pub async fn write_cover_image_to_file(file_path: String, image_data: Buffer) -> Result<()> {
  util::write_cover_image_to_file(file_path, image_data.to_vec())
    .await
    .map_err(napi::Error::from_reason)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_tags() {
    let tags = ApiAudioTags {
      title: Some("Test".to_string()),
      artists: Some(vec!["Test".to_string()]),
      album: Some("Test".to_string()),
      year: Some(2021),
      genre: Some("Test".to_string()),
      track: Some(ApiPosition {
        no: Some(1),
        of: Some(12),
      }),
      album_artists: Some(vec!["Test".to_string()]),
      comment: Some("Test".to_string()),
      disc: Some(ApiPosition {
        no: Some(1),
        of: Some(12),
      }),
      image: Some(ApiImage {
        data: Buffer::from(vec![0x00, 0x01, 0x02, 0x03]),
        mime_type: Some("image/jpeg".to_string()),
        description: Some("Test".to_string()),
      }),
    };
    assert_eq!(tags.title, Some("Test".to_string()));
    assert_eq!(tags.artists, Some(vec!["Test".to_string()]));
    assert_eq!(tags.album, Some("Test".to_string()));
    assert_eq!(tags.year, Some(2021));
    assert_eq!(tags.genre, Some("Test".to_string()));
  }
}
