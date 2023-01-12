#[derive(Debug)]
pub enum FontId {
	Default = 0,
	Huge    = 1,
	Mono    = 2,
}

impl From<&str> for FontId {
	fn from(s: &str) -> Self {
		match s {
			"default" => FontId::Default,
			"huge" => FontId::Huge,
			"mono" => FontId::Mono,
			_ => FontId::Default,
		}
	}
}
