mod tree_gen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "./data/tide.csv";
    tree_gen::generate_tree_from_csv(path)?;

    Ok(())
}
