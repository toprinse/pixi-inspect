use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use std::env;
use std::time::SystemTime;
use tokio::io::AsyncReadExt;
use tokio::fs;
use rattler_conda_types::package::IndexJson;
use rattler_package_streaming::seek as rattler_seek;

#[derive(Parser)]
#[command(name = "pixi-inspect")]
#[command(about = "Extract metadata from a single conda package (index.json) or from a directory")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read metadata from a single .conda file or stdin ("-")
    GetInfo {
        /// Path to .conda file or "-" to read from stdin
        path: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GetInfo { path } => {
            // Resolve input: file on disk or read stdin to a temp file
            let input_path: PathBuf;
            let mut remove_after = false;
            if path == "-" {
                let mut stdin = tokio::io::stdin();
                let mut buf = Vec::new();
                stdin.read_to_end(&mut buf).await?;

                // Create a temporary file with a unique identifier
                let nanos = SystemTime::now().duration_since(std::time::UNIX_EPOCH)?;
                let tmp_name = format!("pixi-inspect-{}.conda", nanos.as_nanos());
                let tmp_path = env::temp_dir().join(tmp_name);
                fs::write(&tmp_path, &buf).await?;
                input_path = tmp_path;
                remove_after = true;
            } else {
                input_path = PathBuf::from(path);
            }

            let index_json: IndexJson = rattler_seek::read_package_file(&input_path)?;

            // Display the JSON
            println!("{}", serde_json::to_string_pretty(&index_json)?);

            // Cleanup temp file if used
            if remove_after {
                let _ = fs::remove_file(input_path).await;
            }

            Ok(())
        }
    }
}