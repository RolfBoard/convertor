mod cli;
mod ffmpeg;
mod plan;
use plan::{ConversionPlan};
use clap::Parser;
use std::path::PathBuf;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Convert {
            input,
            output,
            preset,
            force,
            audio_only,
            video_only,
            format,
            no_metadata,
        } => {
            let mut plan = if let Some(preset) = preset {
                preset.to_plan(PathBuf::from(&input), PathBuf::from(&output))
            } else {
                let mut p = ConversionPlan::default();
                p.input = PathBuf::from(&input);
                p.output = PathBuf::from(&output);
                p
            };

            // overrides vindos da CLI
            if audio_only {
                plan.audio_only = true;
                plan.video_only = false;
            }

            if video_only {
                plan.video_only = true;
                plan.audio_only = false;
            }

            if no_metadata {
                plan.strip_metadata = true;
            }

            if let Some(format) = format {
                plan.apply_format(format);
            }

            println!("{:#?}", plan);

            plan.overwrite = force;

            // normaliza extensão
            plan.normalize_output_extension();

            if plan.output.exists() && !force {
                eprintln!(
                    "Erro: o arquivo '{}' já existe. Use --force para sobrescrever.",
                    plan.output.display()
                );
                std::process::exit(1);
            }

            if let Err(err) = ffmpeg::run(&plan) {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
        Commands::Doctor => {
            if let Err(err) = cli::doctor() {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
        Commands::Completion { shell } => {
            cli::completion(shell);
        }
    }
}
