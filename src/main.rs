#![deny(unsafe_code)]

use anyhow::{anyhow, Result};

mod logger;
use log::{debug, info, warn};
use logger::setup_logger;

use dunce::canonicalize;
use std::path::PathBuf;

use structopt::StructOpt;

use tree_magic_mini::match_filepath;

mod opt_levels;
use opt_levels::*;

use std::process::{Command, Stdio};

#[derive(StructOpt, Debug)]
#[structopt(name = "LosslessOptimiser")]
struct Opt {
    /// Optimisation level, from 0 to 9
    /// Higher numbers will result in increasingly smaller files but will also take increasingly longer to complete.
    /// If ommitted, the default value of 6 will be used.
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
    setup_logger(1).await?;

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
        opt_level = 6;
    } else {
        opt_level = opt.req_opt_level.unwrap();
    }

    // oxipng optimisation args
    let mut oxipng_args = get_oxipng_options(opt_level).await?;
    if opt.verbose {
        oxipng_args.push_str("-v ")
    }

    // ect optimisation args
    let mut ect_args = get_ect_options(opt_level).await?;

    // jpegtran
    let mut jpegtran_args = get_jpegtran_options(opt_level).await?;
    if opt.verbose {
        jpegtran_args.push_str("-verbose ")
    }

    // For each file given to us
    for file in opt.files.iter() {
        let file = canonicalize(file)?;
        let file_path = file.as_path();

        // check if the file extension is a .png
        if file.extension().unwrap() == "png" {
            // now check that the file is actually a png and if not, show a warning and skip the file
            if match_filepath("image/png", file_path) {
                info!("{:?} is a valid (A)PNG file", file);

                // Run it through oxipng first to optimise it, then ECT

                oxipng_args.push_str(file.file_name().unwrap().to_str().unwrap());
                debug!("oxipng_args: {:#?}", oxipng_args);

                if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .arg("/c")
                        .arg("oxipng.exe")
                        .args(oxipng_args.split(' '))
                        .stderr(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .output()
                        .expect("oxipng.exe missing");
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg("oxipng")
                        .args(oxipng_args.split(' '))
                        .stderr(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .output()
                        .expect(
                            "oxipng missing, please install it with your preferred package manager",
                        );
                };

                ect_args.push(' ');
                ect_args.push_str(file.file_name().unwrap().to_str().unwrap());
                debug!("ect_args: {:#?}", ect_args);

                if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .arg("/c")
                        .arg("ect.exe")
                        .args(ect_args.split(' '))
                        .stderr(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .output()
                        .expect("ect.exe missing");
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg("ect")
                        .args(ect_args.split(' '))
                        .stderr(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .output()
                        .expect(
                            "ECT missing, please install it with your preferred package manager",
                        );
                };
            } else {
                warn!(
                    "{:?} has a \".png\" file extension but ISN'T a valid (A)PNG file!",
                    file
                );
                warn!("Skipping file with incorrect extension");
                continue;
            }
        } else if file.extension().unwrap() == "jpg" || file.extension().unwrap() == ".jpeg" {
            if match_filepath("image/jpeg", file_path) {
                info!("{:?} is a valid JPG file", file);

                jpegtran_args.push_str("-outfile ");
                jpegtran_args.push_str(file.file_name().unwrap().to_str().unwrap());
                jpegtran_args.push(' ');
                jpegtran_args.push_str(file.file_name().unwrap().to_str().unwrap());
                debug!("jpegtran_args: {:#?}", jpegtran_args);

                if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .arg("/c")
                        .arg("jpegtran.exe")
                        .args(jpegtran_args.split(' '))
                        .stderr(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .output()
                        .expect("jpegtran.exe missing");
                } else {
                    Command::new("sh")
                            .arg("-c")
                            .arg("jpegtran")
                            .args(jpegtran_args.split(' '))
                            .stderr(Stdio::inherit())
                            .stdout(Stdio::inherit())
                            .output()
                            .expect("jpegtran missing, please install it with your preferred package manager");
                };

                ect_args.push_str(file.file_name().unwrap().to_str().unwrap());
                debug!("ect_args: {:#?}", ect_args);

                if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .arg("/c")
                        .arg("ect.exe")
                        .args(ect_args.split(' '))
                        .stderr(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .output()
                        .expect("ect.exe missing");
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg("ect")
                        .args(ect_args.split(' '))
                        .stderr(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .output()
                        .expect(
                            "ECT missing, please install it with your preferred package manager",
                        );
                };
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
