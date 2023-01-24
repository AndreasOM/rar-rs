use oml_game::system::System;
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct WorldList {
	worlds: Vec<WorldListEntry>,
}

#[derive(Debug, Default)]
pub struct WorldListEntry {
	id:   String,
	name: String,
}

impl WorldListEntry {
	pub fn id(&self) -> &str {
		&self.id
	}
	pub fn name(&self) -> &str {
		&self.name
	}
}

impl WorldList {
	pub fn from_config_asset(system: &mut System, name: &str) -> Option<Self> {
		let dfs = system.default_filesystem_mut();
		// try yaml
		let name_yaml = format!("{}.world_list_config.yaml", &name);
		if dfs.exists(&name_yaml) {
			let mut f = dfs.open(&name_yaml);
			// :TODO: check is_valid ?
			let yaml = f.read_as_string();
			Some(Self::from_yaml(&yaml))
		} else {
			// :TODO: create fallback?
			None
		}
	}
	pub fn worlds(&self) -> &Vec<WorldListEntry> {
		&self.worlds
	}
	fn from_yaml(yaml: &str) -> Self {
		let value: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();

		Self::from_yaml_value(value)
	}

	fn from_yaml_value(yaml_value: serde_yaml::Value) -> Self {
		let config: WorldListConfig = serde_yaml::from_value(yaml_value.clone()).unwrap();
		config.into()
	}
}

impl From<WorldListConfig> for WorldList {
	fn from(c: WorldListConfig) -> Self {
		Self {
			worlds: c.worlds.into_iter().map(|c| c.into()).collect(),
		}
	}
}

impl From<WorldListConfigEntry> for WorldListEntry {
	fn from(c: WorldListConfigEntry) -> Self {
		Self {
			id:   c.id.clone(),
			name: c.name.unwrap_or("".to_string()).clone(),
		}
	}
}

#[derive(Debug, Deserialize)]
struct WorldListConfig {
	worlds: Vec<WorldListConfigEntry>,
}

#[derive(Debug, Deserialize)]
struct WorldListConfigEntry {
	id:   String,
	name: Option<String>,
}
