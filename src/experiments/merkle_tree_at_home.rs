use csv::{Error, StringRecord};
use ethers::{
    abi::{encode, ethabi::Bytes, Token},
    types::H160,
    utils::{hex, keccak256, to_checksum},
};
use rs_merkle::{Hasher, MerkleTree};

use crate::tree_gen;

pub struct MerkleTreeAtHome {
    layers: Vec<Vec<[u8; 32]>>,
}

#[derive(Clone)]
pub struct Keccak256 {}

impl Hasher for Keccak256 {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        keccak256(data)
    }
}

impl MerkleTreeAtHome {
    fn hash_pair(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
        // if left > right swap them
        // (sortPairs = true)

        // let (left, right) = if left > right {
        //     (right, left)
        // } else {
        //     (left, right)
        // };

        // encode left and right together
        let encoded = encode(&[
            Token::FixedBytes(left.to_vec()),
            Token::FixedBytes(right.to_vec()),
        ]);

        return keccak256(encoded);
    }

    pub fn generate(leaves: Vec<[u8; 32]>) -> Self {
        let zero_hash = [0u8; 32];
        let mut layers: Vec<Vec<[u8; 32]>> = vec![];
        layers.push(leaves);

        let mut layer_id = 0;
        while layers.get(layer_id).unwrap().len() > 1 {
            let mut i = 0;
            layers.push(vec![]);
            while i < layers[layer_id].len() {
                let left = layers[layer_id][i];
                // index i+1 can be out of bounds, in that case use zero hash
                let right = layers[layer_id].get(i + 1).unwrap_or(&zero_hash);
                let parent = Self::hash_pair(left, *right);
                layers[layer_id + 1].push(parent);
                i += 2;
            }

            layer_id += 1;
        }

        Self { layers: layers }
    }

    fn get_leaves(&self) -> Option<Vec<[u8; 32]>> {
        Some(self.layers.first()?.to_vec())
    }

    fn get_root(&self) -> Option<[u8; 32]> {
        Some(*self.layers.last()?.first()?)
    }

    fn get_hex_root(&self) -> Option<String> {
        Some(hex::encode(self.get_root()?))
    }

    // pub fn export_to_csv(&self) -> String {
    //     let mut csv = String::new();
    //     for layer in self.layers.iter() {
    //         for leaf in layer.iter() {
    //             csv.push_str(&format!("{:?},", leaf));
    //         }
    //         csv.push_str("\n");
    //     }
    // }
}

#[test]
fn base_tree_test() {
    let path = "./data/test_audience.csv";
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut leaves: Vec<[u8; 32]> = vec![];

    let mut tree: MerkleTree<Keccak256> = MerkleTree::new();

    for result in reader.records() {
        let record = tree_gen::parse_leaf(result).unwrap();
        let hash = keccak256(record);
        leaves.push(hash);
        tree.insert(hash);
    }

    tree.commit();

    let tree_at_home = MerkleTreeAtHome::generate(leaves);

    println!("root_at_home: {:x?}", tree_at_home.get_hex_root().unwrap());
    println!("benchmark_root: {:x?}", tree.root_hex().unwrap());
}
