use std::boxed::Box;
use std::error::Error as StdError;
use std::{fs, io};

fn print_tree(node: &mo_gltf::Node, depth: i32) {
    for _ in 0..(depth - 1) {
        print!("  ");
    }
    print!(" -");
    print!(" Node {}", node.index());
    #[cfg(feature = "names")]
    print!(" ({})", node.name().unwrap_or("<Unnamed>"));
    println!();

    for child in node.children() {
        print_tree(&child, depth + 1);
    }
}

fn run(path: &str) -> Result<(), Box<dyn StdError>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let gltf = mo_gltf::Gltf::from_reader(reader)?;
    for scene in gltf.scenes() {
        print!("Scene {}", scene.index());
        #[cfg(feature = "names")]
        print!(" ({})", scene.name().unwrap_or("<Unnamed>"));
        println!();
        for node in scene.nodes() {
            print_tree(&node, 1);
        }
    }
    Ok(())
}

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run(&path).expect("runtime error");
    } else {
        println!("usage: gltf-tree <FILE>");
    }
}
