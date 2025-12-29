use std::process::Command;
use crate::plan::ConversionPlan;

pub fn run(plan: &ConversionPlan) -> Result<(), String> {
    let status = Command::new("ffmpeg")
        .args(plan.to_ffmpeg_args())
        .status()
        .map_err(|e| format!("Falha ao executar ffmpeg: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err("Erro durante a conversão (ffmpeg retornou erro)".into())
    }
}