fn main() {
	println!("cargo:rerun-if-changed=shaders/");
	let mut options = fs_extra::dir::CopyOptions::new();
	options.overwrite = true;
	let path = std::env::var_os("OUT_DIR").unwrap();
	fs_extra::dir::copy("shaders", path, &options).unwrap();
}
