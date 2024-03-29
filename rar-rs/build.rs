use std::process::Command;

use chrono::prelude::*;

fn main() {
	let utc_now = Utc::now();
	println!(
		"cargo:rustc-env=BUILD_DATETIME={}",
		utc_now.to_rfc3339_opts(SecondsFormat::Secs, true)
	);
	let build_number_path = "../build_number.txt";
	let code_build_number =
		std::fs::read_to_string(build_number_path).expect("Couldn't read build_number file");
	println!("cargo:rustc-env=CODE_BUILD_NUMBER={}", code_build_number);

	let result = Command::new("git")
		.args(["rev-parse", "HEAD"])
		.output()
		.unwrap();
	let git_commit_hash = &String::from_utf8_lossy(&result.stdout)[..7];
	println!("cargo:rustc-env=GIT_COMMIT_HASH={}", git_commit_hash);
}
