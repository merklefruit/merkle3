use csv::{Error, StringRecord};
use ethers::{
    abi::{encode, ethabi::Bytes, Token},
    types::H160,
    utils::{hex, keccak256, to_checksum},
};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use rs_merkle::{Hasher, MerkleTree};
use std::{error::Error as StdError, fmt::Write, process, sync::Arc};

#[derive(Clone)]
pub struct Keccak256 {}

impl Hasher for Keccak256 {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        keccak256(data)
    }
}

fn parse_leaf(line: Result<StringRecord, Error>) -> Result<Bytes, Error> {
    let line = line?;
    // todo: remember to change to get(1) and get(2) with active_users
    let address_raw = line.get(1).unwrap()[2..].to_string();
    let label = line.get(2).unwrap().to_string();

    let address = &hex::decode(address_raw).unwrap();
    let address = to_checksum(&H160::from_slice(address), None);
    let address = H160::from_slice(&hex::decode(&address[2..]).unwrap());

    // return solidity abi-encoded leaf
    Ok(encode(&[Token::Address(address), Token::String(label)]))
}

pub fn generate_tree_from_csv(path: &str) -> Result<MerkleTree<Keccak256>, Box<dyn StdError>> {
    let time = std::time::Instant::now();
    let total_size = std::fs::metadata(path)?.len();
    let line_size = 57;
    let mut count = 0;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.0}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    // todo: sortPairs?
    let mut tree: MerkleTree<Keccak256> = MerkleTree::new();

    let mut reader = csv::Reader::from_path(path)?;

    let mut hashes: Vec<[u8; 32]> = vec![];

    for result in reader.records() {
        let line = parse_leaf(result).unwrap_or_else(|err| {
            eprintln!("Error parsing line: {}", err);
            process::exit(1);
        });

        let hash = Keccak256::hash(line.as_slice());
        hashes.push(hash);

        tree.insert(hash);
        count += 1;

        if count % 1000 == 0 {
            pb.inc(line_size * 1000);
        }
    }

    println!(
        "{} records processed in {:?}, now building tree...",
        count,
        time.elapsed()
    );

    tree.commit();

    println!("Root: {} ", tree.root_hex().unwrap());
    println!("Done building Merkle tree in {:?}", time.elapsed());

    // Generate proof for every leaf:
    let mut proofs: Vec<Vec<String>> = vec![];

    for (i, _) in hashes.iter().enumerate() {
        let proof = Arc::new(tree.proof(&[i]));
        proofs.push(proof.proof_hashes_hex());
    }

    println!("Done generating proofs in {:?}", time.elapsed());

    Ok(tree)
}
