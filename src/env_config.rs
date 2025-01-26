use std::{
    fs,
    io::{self, BufRead, BufReader, ErrorKind},
    path::Path,
};

pub fn config<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let file = match fs::File::open(path.as_ref()) {
        Ok(f) => f,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            eprintln!("ERROR: file: '{}' not found.", path.as_ref().display());
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.trim_start().starts_with('#') {
            continue;
        }
        if let Some((left, right)) = line.split_once('=') {
            let left = left.trim();
            if left.is_empty() {
                continue;
            }
            std::env::set_var(left, right);
        }
    }
    Ok(())
}
