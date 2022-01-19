use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;
use std::str;

fn read_file() -> std::io::Result<()> {
    let mut file = File::open("/tmp/foo")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("contents: {}", contents);

    Ok(())
}

fn seek_file() -> std::io::Result<()> {
    let mut file = File::open("/tmp/foo")?;
    let mut file_copy = file.try_clone()?;

    file.seek(SeekFrom::Start(5))?;

    //let mut contents = vec![];
    let mut contents = String::new();
    //file_copy.read_to_end(&mut contents)?;
    file_copy.read_to_string(&mut contents)?;
    println!("end: {}", contents);

    Ok(())
}

fn run_command() -> std::io::Result<()> {
    let file = File::open("/tmp/foo").unwrap();
    let reverse = Command::new("rev")
        .stdin(file)
        .output()
        .expect("failed reverse command");

    println!("reversed: {:?}", str::from_utf8(&reverse.stdout).unwrap());

    let file = File::open("/tmp/foo").unwrap();
    let lines_of_file = Command::new("wc")
        .arg("-l")
        .stdin(file)
        .output()
        .expect("failed wc command");

    println!(
        "lines of file: {}",
        str::from_utf8(&lines_of_file.stdout).unwrap()
    );

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_from_large_file() -> std::io::Result<()> {
    if let Ok(lines) = read_lines("/tmp/large") {
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("/tmp/foo")?;
    file.write_all(b"some string on file\n")?;

    match read_file() {
        Ok(()) => println!("success read file!"),
        _ => println!("error read file"),
    };

    match seek_file() {
        Ok(()) => println!("success seek file"),
        _ => println!("error seek file"),
    };

    match run_command() {
        Ok(()) => println!("success run command"),
        _ => println!("error run command"),
    };

    read_from_large_file();

    Ok(())
}
