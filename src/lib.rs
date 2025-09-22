#![deny(clippy::all)]

mod util;

use crate::util::{AudioImageType, AudioTags, Image, Position};
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

#[napi(js_name = "AudioImageType", string_enum)]
pub enum ApiAudioImageType {
  Icon,
  OtherIcon,
  CoverFront,
  CoverBack,
  Leaflet,
  Media,
  LeadArtist,
  Artist,
  Conductor,
  Band,
  Composer,
  Lyricist,
  RecordingLocation,
  DuringRecording,
  DuringPerformance,
  ScreenCapture,
  BrightFish,
  Illustration,
  BandLogo,
  PublisherLogo,
  Other,
}

impl ApiAudioImageType {
  pub fn from_audio_image_type(audio_image_type: AudioImageType) -> Self {
    match audio_image_type {
      AudioImageType::Icon => Self::Icon,
      AudioImageType::OtherIcon => Self::OtherIcon,
      AudioImageType::CoverFront => Self::CoverFront,
      AudioImageType::CoverBack => Self::CoverBack,
      AudioImageType::Leaflet => Self::Leaflet,
      AudioImageType::Media => Self::Media,
      AudioImageType::LeadArtist => Self::LeadArtist,
      AudioImageType::Artist => Self::Artist,
      AudioImageType::Conductor => Self::Conductor,
      AudioImageType::Band => Self::Band,
      AudioImageType::Composer => Self::Composer,
      AudioImageType::Lyricist => Self::Lyricist,
      AudioImageType::RecordingLocation => Self::RecordingLocation,
      AudioImageType::DuringRecording => Self::DuringRecording,
      AudioImageType::DuringPerformance => Self::DuringPerformance,
      AudioImageType::ScreenCapture => Self::ScreenCapture,
      AudioImageType::BrightFish => Self::BrightFish,
      AudioImageType::Illustration => Self::Illustration,
      AudioImageType::BandLogo => Self::BandLogo,
      AudioImageType::PublisherLogo => Self::PublisherLogo,
      _ => Self::Other,
    }
  }

  pub fn into_audio_image_type(self) -> AudioImageType {
    match self {
      Self::Icon => AudioImageType::Icon,
      Self::OtherIcon => AudioImageType::OtherIcon,
      Self::CoverFront => AudioImageType::CoverFront,
      Self::CoverBack => AudioImageType::CoverBack,
      Self::Leaflet => AudioImageType::Leaflet,
      Self::Media => AudioImageType::Media,
      Self::LeadArtist => AudioImageType::LeadArtist,
      Self::Artist => AudioImageType::Artist,
      Self::Conductor => AudioImageType::Conductor,
      Self::Band => AudioImageType::Band,
      Self::Composer => AudioImageType::Composer,
      Self::Lyricist => AudioImageType::Lyricist,
      Self::RecordingLocation => AudioImageType::RecordingLocation,
      Self::DuringRecording => AudioImageType::DuringRecording,
      Self::DuringPerformance => AudioImageType::DuringPerformance,
      Self::ScreenCapture => AudioImageType::ScreenCapture,
      Self::BrightFish => AudioImageType::BrightFish,
      Self::Illustration => AudioImageType::Illustration,
      Self::BandLogo => AudioImageType::BandLogo,
      Self::PublisherLogo => AudioImageType::PublisherLogo,
      _ => AudioImageType::Other,
    }
  }
}

#[napi(js_name = "Image", object)]
pub struct ApiImage {
  pub data: Buffer,
  pub pic_type: ApiAudioImageType,
  pub mime_type: Option<String>,
  pub description: Option<String>,
}

impl ApiImage {
  pub fn from_image(image: Image) -> Self {
    Self {
      data: Buffer::from(image.data),
      pic_type: ApiAudioImageType::from_audio_image_type(image.pic_type),
      mime_type: image.mime_type,
      description: image.description,
    }
  }

  pub fn into_image(self) -> Image {
    Image {
      data: self.data.to_vec(),
      pic_type: self.pic_type.into_audio_image_type(),
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
  pub all_images: Option<Vec<ApiImage>>,
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
      all_images: audio_tags
        .all_images
        .map(|images| images.into_iter().map(ApiImage::from_image).collect()),
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
      all_images: self
        .all_images
        .map(|images| images.into_iter().map(ApiImage::into_image).collect()),
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
