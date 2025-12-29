#[derive(Debug, Clone)]
pub enum Container {
    Mp3,
    M4a,
    Aac,
    Wav,
    Ogg,
    Opus,
    Mp4,
    Mov,
    Mkv,
    Webm,
    Avi,
}

#[derive(Debug, Clone)]
pub enum AudioCodec {
    Mp3,
    Aac,
    Opus,
    Pcm,
}

#[derive(Debug, Clone)]
pub enum VideoCodec {
    H264,
    H265,
    Vp9,
}

impl Container {
    pub fn as_str(&self) -> &'static str {
        match self {
            Container::Mp3 => "mp3",
            Container::M4a => "m4a",
            Container::Aac => "aac",
            Container::Wav => "wav",
            Container::Ogg => "ogg",
            Container::Opus => "opus",
            Container::Mp4 => "mp4",
            Container::Mov => "mov",
            Container::Mkv => "mkv",
            Container::Webm => "webm",
            Container::Avi => "avi",
        }
    }
}

impl AudioCodec {
    pub fn as_ffmpeg_codec(&self) -> &'static str {
        match self {
            AudioCodec::Mp3 => "libmp3lame",
            AudioCodec::Aac => "aac",
            AudioCodec::Opus => "libopus",
            AudioCodec::Pcm => "pcm_s16le",
        }
    }
}

impl VideoCodec {
    pub fn as_ffmpeg_codec(&self) -> &'static str {
        match self {
            VideoCodec::H264 => "libx264",
            VideoCodec::H265 => "libx265",
            VideoCodec::Vp9 => "libvpx-vp9",
        }
    }
}