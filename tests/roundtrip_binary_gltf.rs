//! Roundtrip test.
//!
//! Read some binary glTF, write it to disk, and compare to the original.
//! The test will succeed if the output is the same as the original.

use std::io::Read;
use std::{boxed, error, fs, io, path};

const SAMPLE_MODELS_DIRECTORY_PATH: &str = "glTF-Sample-Assets/Models";

fn run() -> Result<(), boxed::Box<dyn error::Error>> {
    let mut all_tests_passed = true;
    let mut nr_test_cases = 0;
    for entry in fs::read_dir(SAMPLE_MODELS_DIRECTORY_PATH)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            if let Some(file_name) = entry_path.file_name() {
                let mut path = entry_path.join("glTF-Binary").join(file_name);
                path.set_extension("glb");
                if path.exists() {
                    // not all models have binary versions
                    if let Err(err) = test(&path) {
                        println!("{:?}: error: {:?}", path, err);
                        all_tests_passed = false;
                    } else {
                        println!("{:?}: ok", path);
                        nr_test_cases += 1;
                    }
                }
            }
        }
    }

    if sparse_accessor_without_buffer_view_test() {
        nr_test_cases += 1;
    } else {
        all_tests_passed = false;
    }

    assert!(all_tests_passed);
    assert!(nr_test_cases >= 25);
    Ok(())
}

fn test(path: &path::Path) -> Result<(), boxed::Box<dyn error::Error>> {
    let file = fs::File::open(path)?;
    let length = file.metadata()?.len() as usize;
    let mut reader = io::BufReader::new(file);
    let mut original = Vec::with_capacity(length);
    reader.read_to_end(&mut original)?;

    // Check from_reader/to_vec implementation.
    {
        let glb = mo_gltf::binary::Glb::from_reader(io::Cursor::new(&original))?;
        let output = glb.to_vec()?;
        assert_eq!(&original, &output);
    }

    // Check from_slice/to_writer implementation.
    {
        let glb = mo_gltf::binary::Glb::from_slice(&original)?;
        let mut output = Vec::with_capacity(length);
        glb.to_writer(&mut output as &mut dyn io::Write)?;
        assert_eq!(&original, &output);
    }

    Ok(())
}

/// Test a file with a sparse accessor with no buffer view.
///
/// Return true if the test passes, and false otherwise.
fn sparse_accessor_without_buffer_view_test() -> bool {
    let path = path::Path::new("tests/box_sparse.glb");
    if let Err(err) = test(path) {
        println!("{:?}: error: {:?}", path, err);
        false
    } else {
        println!("{:?}: ok", path);
        true
    }
}

#[test]
fn roundtrip_binary_gltf() {
    run().expect("test failure");
}
