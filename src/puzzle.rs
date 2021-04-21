//! puzzle.rs to retrieve challenge inputs
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path
};

use curl::easy::{Easy2, Handler, WriteError};
use dotenv::dotenv;
use anyhow::{bail, Result};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn get_puzzle(year: u16, day: u8) -> Result<String, anyhow::Error> {
    let input = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let output = format!("inputs/{}/{}.in", year, day);

    // fetch data and write to file if necessary
    if !Path::new(&output).exists() {
        get_input(&input, &output)?;
    } 

    Ok(read_file(&output))
}

/// curl is used here to make a request with a session cookie (required)
fn get_input(input: &str, output: &str) -> Result<(), anyhow::Error>
{
    dotenv().ok();
    if let Some(session) = std::env::var("SESSION").ok() {
        let mut easy = Easy2::new(Collector(Vec::new()));
        easy.get(true)?;
        easy.url(&input)?;
        easy.cookie(&format!("session={}", session))?;
        easy.perform()?;
        match easy.response_code()? {
            200 => {
                let contents = easy.get_ref();
                write_file(&contents.0, &output);
            },
            _ => bail!("HTTP response error"),
        } 
    }
    else {
        bail!("Missing session variable");
    }

    Ok(())
}

fn read_file(filepath: &str) -> String {
    let file = File::open(filepath).expect("Unable to open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    buffered_reader
        .read_to_string(&mut contents)
        .expect("Unable to read file into the string");
    contents.trim_end().to_string()
}

fn write_file(contents: &[u8], filepath: &str) {
    let file = File::create(filepath).expect("Unable to create file");
    let mut buffered_writer = BufWriter::new(file);
    buffered_writer
        .write_all(contents)
        .expect("Unable to write to file");
}