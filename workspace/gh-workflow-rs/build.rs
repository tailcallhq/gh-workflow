use std::path::Path;
use std::process::Command;

fn main() {
    let cur_dir = env!("CARGO_MANIFEST_DIR");
    // Path to the schema file
    // let schema_path = Path::new("github-workflow.json");
    let schema_path = Path::new(cur_dir).join("schema.json");

    // Output path for the generated Rust file
    let output_path = Path::new(cur_dir).join("src").join("model.rs");

    // Run Quicktype to generate the Rust code from the schema
    let result = Command::new("quicktype")
        .arg("--src")
        .arg(&schema_path)
        .arg("--src-lang")
        .arg("json")
        .arg("--lang")
        .arg("rust")
        .arg("--out")
        .arg(output_path)
        .status()
        .expect("Failed to run quicktype");

    // Ensure the command succeeded
    if !result.success() {
        panic!("Quicktype generation failed!");
    }

    /*if !postprocess_result.success() {
        panic!("Post-processing failed!");
    }*/

    // Rebuild if the schema changes
    println!("cargo:rerun-if-changed={}", schema_path.display());
}
