mod codecs;
mod commands;
mod error;
mod resource;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Increase the default log level, can be added multiple times
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbosity: u8,

    /// Directory in which to output the decoded/encoded files
    #[arg(short, long, global = true)]
    output_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract a VT7 file into its components
    ///
    /// If the file is an archive file (.vt7a or .osa), this will create a .json
    /// file and a folder. The folder contains all files from inside the archive
    /// and the json file contains a list of all files. The json file is needed
    /// to be able to encode everything back together.
    ///
    /// If the file is anything else, this will try to convert it into a useful
    /// format (if applicable).
    Decode {
        /// Path to the VT7 file
        filepath: PathBuf,

        /// Output all files (even those who are not yet supported)
        #[arg(short, long, global = true)]
        all: bool,
    },
    /// Pack components into a valid VT7 file
    ///
    /// This does the opposite direction of decode. The main usecase is to pack
    /// together files into an archive (.vt7a or .osa). To do that, specify the
    /// path to the archive json file which contains the listing of all archive
    /// contents (this should've been created in `decode`). Afterwards the new
    /// archive will be created in the output directory.
    Encode {
        /// Path to the json file of a decoded VT7 file
        filepath: PathBuf,
    },
    /// Print statistics about a valid VT7 file
    ///
    /// This command counts the amount of files included in an VT7 archive and
    /// displays them grouped by format and extension.
    Stats {
        /// Path to the VT7 file
        filepath: PathBuf,
    },
    /// Compare two VT7 files for differences
    ///
    /// This command takes two VT7 archives and compares their contents. It will
    /// report which files are present in only one archive or which files are
    /// present in both archives, but have different contents.
    Diff {
        /// Path to the first VT7 file
        filepath1: PathBuf,
        /// Path to the second VT7 file
        filepath2: PathBuf,
    },
}

fn setup_logger(verbosity: u8) -> Result<(), fern::InitError> {
    let level = match verbosity {
        1 => log::LevelFilter::Debug,
        2 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };

    fern::Dispatch::new()
        .format(|out, message, record| out.finish(format_args!("[{}] {}", record.level(), message)))
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    setup_logger(cli.verbosity)?;

    let outpath: PathBuf = cli
        .output_dir
        .unwrap_or_else(|| std::path::Path::new("out/").to_path_buf());

    if std::fs::create_dir_all(outpath.clone()).is_err() {
        return Err(Box::new(error::DecodingError::ParsingError(format!(
            "Output directory ({}) does not exist and could not be created",
            outpath.display()
        ))));
    }

    match &cli.command {
        Commands::Decode { filepath, all } => {
            commands::decode(filepath, &outpath, *all)?;
        }
        Commands::Encode { filepath } => {
            commands::encode(filepath, &outpath)?;
        }
        Commands::Stats { filepath } => {
            commands::statistics(filepath)?;
        }
        Commands::Diff {
            filepath1,
            filepath2,
        } => {
            commands::diff(filepath1, filepath2)?;
        }
    }

    Ok(())
}
