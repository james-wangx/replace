use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Not enough arguments.");
        process::exit(1);
    }

    let filename = &args[1];
    let target = &args[2];
    let switch = &args[3];

    if let Err(e) = replace(filename, target, switch) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    
}

fn replace(filename: &str, target: &str, switch: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let results = contents.replace(target, switch);
    fs::write(filename, results)?;

    Ok(())
}
