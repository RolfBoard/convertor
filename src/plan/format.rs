use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Format {
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