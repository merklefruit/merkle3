use csv::{Error, StringRecord};
use ethers::utils::keccak256;
use rs_merkle::{Hasher, MerkleTree};
use std::process;

#[derive(Clone)]
struct Keccak256 {}

impl Hasher for Keccak256 {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        keccak256(data)
    }
}

fn parse_line(line: Result<StringRecord, Error>) -> Result<String, Error> {
    let line = line?;
    let address = line.get(1).unwrap().to_string();
    let label = line.get(2).unwrap().to_string();

    Ok(format!("{}{}", address, label))
}

fn main() {
    let path = "./data/active_users_audience.csv";
    let time = std::time::Instant::now();
    let mut count = 0;

    let mut tree: MerkleTree<Keccak256> = MerkleTree::new();

    match csv::Reader::from_path(path) {
        Ok(mut reader) => {
            for result in reader.records() {
                let line = parse_line(result).unwrap_or_else(|err| {
                    eprintln!("Error parsing line: {}", err);
                    process::exit(1);
                });

                let hash = Keccak256::hash(line.as_bytes());

                tree.insert(hash);
                count += 1;

                if count % 100000 == 0 {
                    println!("{} records processed", count);
                }
            }
        }

        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }

    println!(
        "{} records processed in {:?}, now building tree...",
        count,
        time.elapsed()
    );

    tree.commit();

    println!("Root: {:?} ", tree.root_hex());
    println!("Done building Merkle tree in {:?}", time.elapsed());
}
