use std::path::PathBuf;
use clap::ValueEnum;
use super::{ConversionPlan, Container, AudioCodec, VideoCodec};

#[derive(Debug, Clone, ValueEnum)]
pub enum Preset {
    Whatsapp,
    Signal,
    Imessage,

    Archive,
    AudioAac,
    AudioOpus,
}

impl Preset {
    pub fn to_plan(&self, input: PathBuf, output: PathBuf) -> ConversionPlan {
        match self {
            Preset::Whatsapp => ConversionPlan {
                input,
                output,
                container: Container::Ogg,
                audio_codec: Some(AudioCodec::Opus),
                video_codec: None,
                audio_bitrate_kbps: Some(64),
                strip_metadata: true,
                audio_only: true,
                video_only: false,
                overwrite: false,
            },

            Preset::Signal => ConversionPlan {
                input,
                output,
                container: Container::Ogg,
                audio_codec: Some(AudioCodec::Opus),
                video_codec: None,
                audio_bitrate_kbps: Some(64),
                strip_metadata: true,
                audio_only: true,
                video_only: false,
                overwrite: false,
            },

            Preset::Imessage => ConversionPlan {
                input,
                output,
                container: Container::M4a,
                audio_codec: Some(AudioCodec::Aac),
                video_codec: None,
                audio_bitrate_kbps: Some(128),
                strip_metadata: false,
                audio_only: true,
                video_only: false,
                overwrite: false,
            },

            Preset::Archive => ConversionPlan {
                input,
                output,
                container: Container::Mov,
                audio_codec: Some(AudioCodec::Aac),
                video_codec: Some(VideoCodec::H265),
                audio_bitrate_kbps: Some(192),
                strip_metadata: false,
                audio_only: false,
                video_only: false,
                overwrite: false,
            },

            Preset::AudioAac => ConversionPlan {
                input,
                output,
                container: Container::Aac,
                audio_codec: Some(AudioCodec::Aac),
                video_codec: None,
                audio_bitrate_kbps: Some(128),
                strip_metadata: true,
                audio_only: true,
                video_only: false,
                overwrite: false,
            },

            Preset::AudioOpus => ConversionPlan {
                input,
                output,
                container: Container::Opus,
                audio_codec: Some(AudioCodec::Opus),
                video_codec: None,
                audio_bitrate_kbps: Some(96),
                strip_metadata: true,
                audio_only: true,
                video_only: false,
                overwrite: false,
            },
        }
    }
}