// SPDX-License-Identifier: MPL-2.0
use std::{
	env,
	path::{Path, PathBuf},
	process::Command,
};

fn gen_windows(link_dir: &Path, out_dir: &Path) {
	let mut lib = cc::windows_registry::find("i686", "lib.exe").expect("failed to find lib.exe");

	let def_path = link_dir.join("windows").join("byondcore.def");
	let out_path = out_dir.join("byondcore.lib");

	let lib_status = lib
		.current_dir(out_dir)
		.arg(format!("/def:{}", def_path.display()))
		.arg("/machine:x86")
		.arg(format!("/out:{}", out_path.display()))
		.status()
		.expect("failed to compile stub byondcore.lib");

	if !lib_status.success() {
		panic!("lib.exe failed");
	}

	println!("cargo:rustc-link-lib=byondcore");
}

fn gen_linux(link_dir: &Path, out_dir: &Path) {
	let link_dir = link_dir.join("linux");
	let stub_path = link_dir.join("stub.c");
	let ver_path = link_dir.join("libbyond.ver");
	let so_path = out_dir.join("libbyond.so");

	let compile_status = Command::new("g++")
		.current_dir(out_dir)
		.args(["-m32", "-fPIC", "-shared", "-O3", "-Wall", "-Wextra"])
		.arg(format!(
			"-Wl,--version-script={}",
			ver_path.to_str().expect("Invalid version script path")
		))
		.arg("-o")
		.arg(so_path)
		.arg(stub_path)
		.status()
		.expect("Failed to compile stub");

	if !compile_status.success() {
		panic!("Compilation failed");
	}

	println!("cargo:rustc-link-lib=byond");
}

fn main() {
	let target_os = env::var("CARGO_CFG_TARGET_OS").expect("no CARGO_CFG_TARGET_OS");
	let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("no CARGO_CFG_TARGET_ARCH");
	let link_dir = env::var("CARGO_MANIFEST_DIR")
		.map(PathBuf::from)
		.expect("no CARGO_MANIFEST_DIR")
		.join("link");
	let out_dir = env::var("OUT_DIR").map(PathBuf::from).expect("no OUT_DIR");

	// the compile_error! in the main crate will result in a cleaner error message
	if target_arch != "x86" {
		return;
	}

	match target_os.as_str() {
		"windows" => gen_windows(&link_dir, &out_dir),
		"linux" => gen_linux(&link_dir, &out_dir),
		_ => return,
	}

	println!("cargo:rustc-link-search={}", out_dir.display());
	println!("cargo:rerun-if-changed=link/stub.cpp");
	println!("cargo:rerun-if-changed=link/windows/byondcore.def");
	println!("cargo:rerun-if-changed=link/linux/byond.ver");
}
