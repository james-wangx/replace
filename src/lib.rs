use std::{error::Error, fs};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The target file or directory
    target: String,

    /// The string to be replace
    string: String,

    /// The result string, after replace
    switch: String,
}

impl Args {
    pub fn replace(&self, target: &str) -> Result<(), Box<dyn Error>> {
        let md = fs::metadata(target)?;

        if md.is_file() {
            let contents = fs::read_to_string(target)?;
            let results = contents.replace(&self.string, &self.switch);
            fs::write(target, results)?;
            println!("Replaced in file {}", target);
            return Ok(());
        } else {
            for entry in fs::read_dir(target)? {
                let path = entry?.path();
                self.replace(path.to_str().unwrap())?;
            }
            Ok(())
        }
    }

    pub fn get_target(&self) -> &str {
        &self.target
    }
}
