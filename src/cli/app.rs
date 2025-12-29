use clap::{Parser, Subcommand};
use clap_complete::Shell;

use crate::plan::{Preset, Format};

#[derive(Parser)]
#[command(
    name = "convertor",
    version,
    about = "CLI para conversão simples de áudio e vídeo",
    long_about = "Ferramenta de linha de comando para conversão de mídia (áudio/vídeo) usando ffmpeg."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Converte um arquivo de mídia
    Convert {
        /// Arquivo de entrada
        input: String,

        /// Arquivo de saída
        output: String,

        /// Preset de conversão
        #[arg(long, value_enum)]
        preset: Option<Preset>,

        /// Força sobrescrita do arquivo de saída
        #[arg(short, long)]
        force: bool,

        /// Extrai apenas o áudio
        #[arg(long)]
        audio_only: bool,

        /// Remove trilha de áudio
        #[arg(long)]
        video_only: bool,

        /// Formato de saída
        #[arg(long, value_enum)]
        format: Option<Format>,

        /// Remove todos os metadados
        #[arg(long)]
        no_metadata: bool,
    },

    /// Verifica dependências do sistema (ffmpeg, etc)
    Doctor,

    /// Gera scripts de autocomplete para shells
    Completion {
        /// Shell alvo
        #[arg(value_enum)]
        shell: Shell,
    },
}