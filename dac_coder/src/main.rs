use hex;
use clap::Parser;
use std::path::PathBuf;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use once_cell::sync::Lazy;
use tezos_encoding::enc::BinWriter;
use tezos_rollup_encoding::dac::{Page, V0ContentPage, V0HashPage, MAX_PAGE_SIZE};
use tezos_core::internal::crypto::Crypto;

const PREIMAGE_HASH_SIZE: usize = 33;
const MAX_DAC_LEVELS: usize = 4;
const MAX_FILE_SIZE: u64 = 10_048_576;
const CRYPTO: Lazy<Crypto> = Lazy::new(|| Crypto::new(None, None, None));

fn hash_digest(preimage: &[u8]) -> [u8; PREIMAGE_HASH_SIZE] {
    let digest_256 = CRYPTO.blake2b(preimage, 32).expect("Failed to calculate hash");
    let mut hash_with_prefix = [0; PREIMAGE_HASH_SIZE];
    hash_with_prefix[1..].copy_from_slice(&digest_256);
    hash_with_prefix
}

fn write_page(page: &Page, output_path: &PathBuf) -> [u8; PREIMAGE_HASH_SIZE] {
    let mut data = Vec::with_capacity(MAX_PAGE_SIZE);
    page.bin_write(&mut data).expect("Failed to serialize content page");

    let hash = hash_digest(data.as_slice());
    let path = output_path.join(hex::encode(hash));
    let mut output_file = File::create(path).expect("Failed to open file for writing");
    output_file.write(data.as_slice()).expect("Failed to write file");
    hash
}

fn ensure_dir_exists(output_path: &PathBuf) {
    if output_path.exists() {
        if !output_path.is_dir() {
            panic!("{:?} is not a directory", output_path);
        }
    } else {
        create_dir_all(output_path).expect("Failed to create output directory");
    }
}

fn read_source_file(source_path: &PathBuf) -> Vec<u8> {
    let file_size = std::fs::metadata(source_path).expect("Failed to find source").len();
    if file_size > MAX_FILE_SIZE {
        panic!("Source file is too large");
    }

    let mut source_file = File::open(source_path).expect("Failed to open source file for reading");
    let mut buffer: Vec<u8> = Vec::with_capacity(file_size.try_into().unwrap());    

    if let Err(error) = source_file.read_to_end(&mut buffer) {
        panic!("Failed to read source file: {:?}", error);
    }

    buffer
}

fn hash_loop(level: usize, pages: &Vec<Page>, hashes: &mut Vec<[u8; PREIMAGE_HASH_SIZE]>, output_path: &PathBuf) -> String {
    if level >= MAX_DAC_LEVELS {
        panic!("DAC preimage tree contains too many levels: {}", level);
    }

    hashes.clear();
    
    for page in pages {
        let hash = write_page(&page, &output_path);
        hashes.push(hash);
    }

    if hashes.len() == 1 {
        hex::encode(hashes[0])
    } else {
        let hashes_pages: Vec<Page> = V0HashPage::new_pages(hashes.as_slice())
            .map(|c| Page::V0HashPage(c))
            .collect();
        hash_loop(level + 1, &hashes_pages, hashes, output_path) 
    }
}

fn generate_pages_v0(source_path: &PathBuf, output_path: &PathBuf) -> String {
    ensure_dir_exists(output_path);

    let input = read_source_file(source_path);    
    let mut hashes: Vec<[u8; PREIMAGE_HASH_SIZE]> = Vec::with_capacity(input.len() / V0ContentPage::MAX_CONTENT_SIZE + 1);
    let pages: Vec<_> = V0ContentPage::new_pages(input.as_slice())
        .map(|p| Page::V0ContentPage(p))
        .collect();

    return hash_loop(0, &pages, &mut hashes, output_path);
}

#[derive(Parser, Debug)]
struct Args {
   /// Path to the file that is to be DAC encoded
   source_file: PathBuf,

   /// Output directory to save hash/content pages
   #[arg(short, long)]
   output_dir: Option<PathBuf>,
}

fn main() {
   let args = Args::parse();

   let output_path = args.output_dir.unwrap_or(PathBuf::from("."));
   let root_hash = generate_pages_v0(&args.source_file, &output_path);

   print!("{}", root_hash);
}