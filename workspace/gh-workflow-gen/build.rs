use gh_workflow::toolchain::RustToolchain;

fn main() {
    let toolchain = RustToolchain::default()
        .workspace(true)
        .fmt(true)
        .clippy(true);
    toolchain
        .to_workflow()
        .unwrap()
        .write(format!(
            "{}/../../.github/workflows/ci.yml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
}
