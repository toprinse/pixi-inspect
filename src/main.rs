use anyhow::Result;
use clap::{Parser, Subcommand};
use rattler_conda_types::package::{ArchiveType, IndexJson, PackageFile};
use rattler_package_streaming::seek::{self as rattler_seek, read_package_file_content};
use std::{io::Read, path::PathBuf};

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GetInfo { path } => {
            // Resolve input: file on disk or read stdin to a temp file
            let index_json: IndexJson = if path == "-" {
                let mut stdin = std::io::stdin();
                let mut buf = Vec::new();
                stdin.read_to_end(&mut buf)?;
                let magic_bytes = &buf[0..4];
                // https://en.wikipedia.org/wiki/List_of_file_signatures
                let archive_type = match magic_bytes {
                    // zip magic number
                    [0x50, 0x4B, 0x03, 0x04]
                    | [0x50, 0x4B, 0x05, 0x06]
                    | [0x50, 0x4B, 0x07, 0x08] => ArchiveType::Conda,
                    // bz2 magic number
                    [0x42, 0x5a, 0x68, _] => ArchiveType::TarBz2,
                    _ => {
                        return Err(anyhow::anyhow!(
                            "Unsupported archive type. Magic bytes {magic_bytes:?} don't match any known format"
                        ));
                    }
                };
                let mut reader = std::io::Cursor::new(buf);
                let content = read_package_file_content(
                    &mut reader,
                    archive_type,
                    IndexJson::package_path(),
                )?;
                IndexJson::from_str(&String::from_utf8_lossy(&content))?
            } else {
                rattler_seek::read_package_file(&PathBuf::from(path))?
            };

            // Display the JSON
            println!("{}", serde_json::to_string_pretty(&index_json)?);

            Ok(())
        }
    }
}
