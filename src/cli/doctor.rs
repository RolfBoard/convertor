use std::process::Command;

pub fn run() -> Result<(), String> {
    match Command::new("ffmpeg").arg("-version").output() {
        Ok(output) => {
            if output.status.success() {
                println!("ffmpeg encontrado:");
                println!("{}", String::from_utf8_lossy(&output.stdout));
                Ok(())
            } else {
                Err("ffmpeg encontrado, mas retornou erro".into())
            }
        }
        Err(_) => Err("ffmpeg não encontrado no PATH".into()),
    }
}