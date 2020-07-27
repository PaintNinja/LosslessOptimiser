#![deny(unsafe_code)]

use anyhow::{anyhow, Result};

mod logger;
use log::{debug, error, info, warn};
use logger::setup_logger;

use std::path::PathBuf;

use structopt::StructOpt;

use tree_magic_mini::match_filepath;

mod opt_levels;
use opt_levels::*;

use std::process::Command;

#[derive(StructOpt, Debug)]
#[structopt(name = "LosslessOptimiser")]
struct Opt {
    /// Optimisation level, from 0 to 9
    /// Higher numbers will result in increasingly smaller files but will also take increasingly longer to complete.
    /// If ommitted, the default value of 5 will be used.
    #[structopt(short = "o", long = "opt-level")]
    req_opt_level: Option<u8>,

    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

#[async_std::main]
async fn main() -> Result<()> {
    setup_logger(2).await?;

    let opt = Opt::from_args();
    println!("{:#?}", opt);

    if opt.files.is_empty() {
        return Err(anyhow!(
            "File argument is required but is missing. Run me with --help for assistance."
        ));
    } else if opt.files[0].to_string_lossy() == "/?" {
        return Err(anyhow!(
            "Sorry, I don't support Windows-style CLI args yet. Try using -h or --help instead."
        ));
    }

    let opt_level;
    if opt.req_opt_level.is_none() {
        opt_level = 5;
    } else {
        opt_level = opt.req_opt_level.unwrap();
    }

    // For each file given to us
    for file in opt.files.iter() {
        let file_path = file.as_path();

        // check if the file extension is a .png
        if file.ends_with(".png") {
            // now check that the file is actually a png and if not, show a warning and skip the file
            if match_filepath("image/png", file_path) {
                info!("{:?} is a valid (A)PNG file", file);
                let mut oxipng_args = get_oxipng_options(opt_level).await?;
                Command::new("cmd").args(&["/c", "oxipng.exe "]);
            } else {
                warn!("{:?} has a \".png\" file extension but ISN'T a valid (A)PNG file!", file);
                warn!("Skipping file with incorrect extension");
                continue;
            }
        } else if file.ends_with(".jpg") || file.ends_with(".jpeg") {
            if match_filepath("image/jpg", file_path) {
                info!("{:?} is a valid JPG file", file);
            } else {
                warn!(
                    "{:?} has a \".jpg\"/\".jpeg\" file extension but ISN'T a valid JPEG file!",
                    file
                );
                warn!("Skipping file with incorrect extension");
                continue;
            }
        } else {
            warn!("Skipping unknown file extension {:?}", file);
        }
    }

    Ok(())
}
