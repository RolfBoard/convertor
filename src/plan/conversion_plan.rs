use std::path::PathBuf;
use super::Format;
use super::codec::{Container, AudioCodec, VideoCodec};

#[derive(Debug, Clone)]
pub struct ConversionPlan {
    pub input: PathBuf,
    pub output: PathBuf,

    pub container: Container,
    pub audio_codec: Option<AudioCodec>,
    pub video_codec: Option<VideoCodec>,
    pub audio_bitrate_kbps: Option<u32>,
    pub strip_metadata: bool,
    pub audio_only: bool,
    pub video_only: bool,
    pub overwrite: bool,
}

impl Default for ConversionPlan {
    fn default() -> Self {
        Self {
            input: PathBuf::new(),
            output: PathBuf::new(),
            container: Container::Mp4,
            audio_codec: None,
            video_codec: None,
            audio_bitrate_kbps: None,
            strip_metadata: false,
            audio_only: false,
            video_only: false,
            overwrite: false,
        }
    }
}

impl ConversionPlan {
    pub fn to_ffmpeg_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        // overwrite behavior
        if self.overwrite {
            args.push("-y".into());
        } else {
            args.push("-n".into());
        }

        // input
        args.push("-i".into());
        args.push(self.input.to_string_lossy().to_string());

        // metadata
        if self.strip_metadata {
            args.push("-map_metadata".into());
            args.push("-1".into());
        }

        // audio / video selection
        if self.audio_only {
            args.push("-vn".into());
        }

        if self.video_only {
            args.push("-an".into());
        }

        // audio codec
        if let Some(codec) = &self.audio_codec {
            args.push("-c:a".into());
            args.push(codec.as_ffmpeg_codec().into());
        }

        // audio bitrate
        if let Some(bitrate) = self.audio_bitrate_kbps {
            args.push("-b:a".into());
            args.push(format!("{}k", bitrate));
        }

        // video codec
        if let Some(codec) = &self.video_codec {
            args.push("-c:v".into());
            args.push(codec.as_ffmpeg_codec().into());
        }

        // output
        args.push(self.output.to_string_lossy().to_string());

        args
    }
    pub fn apply_format(&mut self, format: Format) {
        match format {
            // Áudio
            Format::Mp3 => {
                self.container = Container::Mp3;
                self.audio_codec = Some(AudioCodec::Mp3);
                self.video_codec = None;
                self.audio_only = true;
            }

            Format::M4a | Format::Aac => {
                self.container = Container::M4a;
                self.audio_codec = Some(AudioCodec::Aac);
                self.video_codec = None;
                self.audio_only = true;
            }

            Format::Wav => {
                self.container = Container::Wav;
                self.audio_codec = Some(AudioCodec::Pcm);
                self.video_codec = None;
                self.audio_only = true;
            }

            Format::Ogg | Format::Opus => {
                self.container = Container::Ogg;
                self.audio_codec = Some(AudioCodec::Opus);
                self.video_codec = None;
                self.audio_only = true;
            }

            // Vídeo
            Format::Mp4 | Format::Mov => {
                self.container = Container::Mp4;
                self.video_codec = Some(VideoCodec::H264);
            }

            Format::Mkv => {
                self.container = Container::Mkv;
                self.video_codec = Some(VideoCodec::H264);
            }

            Format::Webm => {
                self.container = Container::Webm;
                self.video_codec = Some(VideoCodec::Vp9);
            }

            Format::Avi => {
                self.container = Container::Avi;
                self.video_codec = Some(VideoCodec::H264);
            }
        }
    }
    pub fn normalize_output_extension(&mut self) {
        let expected_ext = self.container.as_str();

        match self.output.extension() {
            Some(ext) => {
                if ext != expected_ext {
                    eprintln!(
                        "Aviso: extensão '{}' pode não bater com o formato! A conversão pode não acontecer...'{}'",
                        ext.to_string_lossy(),
                        expected_ext
                    );
                }
            }
            None => {
                self.output.set_extension(expected_ext);
            }
        }
    }
}