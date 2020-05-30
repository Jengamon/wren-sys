fn main() {
    cc::Build::new()
        .files(glob::glob("src/*.c").expect("Failed to glob c files").filter_map(|x| x.ok()))
        .include("include")
        .include("optional")
        .compile("wren");
}