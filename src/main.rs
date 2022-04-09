use std::{
    env,
    error::Error,
    fs::{self, metadata},
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Not enough arguments.");
        process::exit(1);
    }

    let target = &args[1];
    let string = &args[2];
    let switch = &args[3];

    if let Err(e) = replace(target, string, switch) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn replace(target: &str, string: &str, switch: &str) -> Result<(), Box<dyn Error>> {
    let md = metadata(target)?;

    if md.is_file() {
        let contents = fs::read_to_string(target)?;
        let results = contents.replace(string, switch);
        fs::write(target, results)?;
        println!("Replace in file {}, done.", target);
        return Ok(());
    } else {
        for entry in fs::read_dir(target)? {
            let path = entry?.path();
            replace(path.to_str().unwrap(), string, switch)?;
        }
        Ok(())
    }
}
