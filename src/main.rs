use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
use std::process;

fn main() {
    let path = "./data/active_users_audience.csv";
    let time = std::time::Instant::now();
    let mut count = 0;

    let mut tree: MerkleTree<Sha256> = MerkleTree::new();

    match csv::Reader::from_path(path) {
        Ok(mut reader) => {
            for result in reader.records() {
                let record = result.unwrap();
                let hash = Sha256::hash(record[0].as_bytes());
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

    tree.commit();

    println!("Root: {:?} ", tree.root_hex());

    println!("Processed {} records in {:?}", count, time.elapsed());
}
