fn main() {
    let artifacts = luajit_src::Build::new().build();
    artifacts.print_cargo_metadata();
    println!("cargo:rerun-if-changed=build.rs");
}
