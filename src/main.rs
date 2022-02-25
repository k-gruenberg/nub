use std::io;
use std::fs;
use std::fs::{DirEntry, File};
use std::io::{BufReader, Error, Read};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use clap::{Parser};
use ring::digest::{Context, Digest, SHA256};
use data_encoding::HEXUPPER;

/// Simple command line tool to delete all duplicate files in a given directory
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Path to a directory containing files
    #[clap(parse(from_os_str))]
    directory: PathBuf,
}

fn main() {
    let args = Args::parse();

    if !args.directory.is_dir() {
        eprintln!("{} is not a directory!", args.directory.display());
        return;
    } else {
        match delete_duplicate_files(args.directory.as_path()) {
            Ok(()) => println!("Successfully deleted duplicate files from {}", args.directory.display()),
            Err(err) => eprintln!("{}", err)
        }
    }
}

fn delete_duplicate_files(directory: &Path) -> io::Result<()> {
    let all_files_in_dir = directory.read_dir()?
        .filter(|dir_entry: &io::Result<DirEntry>| dir_entry.is_ok())
        .map(|dir_entry: io::Result<DirEntry>| dir_entry.unwrap().path())
        .filter(|path: &PathBuf| path.is_file());
    let all_files_in_dir_with_hash: Vec<(String, PathBuf)> = all_files_in_dir
        .map(|file: PathBuf| (file_hash(&file), file))
        .filter(|(file_hash, _file)| file_hash.is_ok())
        .map(|(file_hash, file)| (file_hash.unwrap(), file))
        .collect();

    let all_hashes= HashSet::<&String>::from_iter(
        all_files_in_dir_with_hash.iter()
            .map(|(file_hash, _file)| file_hash)
    );
    for hash in all_hashes {
        let files_with_that_hash: Vec<&PathBuf> = all_files_in_dir_with_hash.iter()
            .filter(|(file_hash, _file)| file_hash == hash)
            .map(|(_file_hash, file)| file)
            .collect();
        for duplicate_file in files_with_that_hash[1..].iter() {
            if let Err(_) = fs::remove_file(duplicate_file) {
                eprintln!("Could not delete file {}", duplicate_file.display());
            } else {
                println!("Deleted duplicate of {}: {}", files_with_that_hash[0].display(), duplicate_file.display());
            }
        }
    }

    Ok(())
}

/// Hashes the content of a given file (computes the digest).
fn file_hash<P: AsRef<Path>>(file_path: P) -> Result<String, Error> {
    // cf. https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);
    let digest: Digest = sha256_digest(reader)?;
    Ok(HEXUPPER.encode(digest.as_ref()))
}

/// Copied from https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}