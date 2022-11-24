use chrono::prelude::*;

fn main() {

	let utc_now = Utc::now();
	println!("cargo:rustc-env=BUILD_DATETIME={}", utc_now.to_rfc3339());
}

