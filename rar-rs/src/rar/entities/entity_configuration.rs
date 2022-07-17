use std::collections::HashMap;

use oml_game::math::Vector2;
use oml_game::system::System;
use serde::{Deserialize, Serialize};

use crate::rar::entities::entity_ids::*;
use crate::rar::entities::EntityType;

#[derive(Debug)]
pub struct AnimatedTextureConfiguration {
	pub prefix:           String,
	pub number_of_digits: i8,
	pub first_frame:      u16,
	pub last_frame:       u16,
	pub fps:              f32,
}

impl AnimatedTextureConfiguration {
	pub fn new(
		prefix: &str,
		number_of_digits: i8,
		first_frame: u16,
		last_frame: u16,
		fps: f32,
	) -> Self {
		Self {
			prefix: prefix.to_owned(),
			number_of_digits,
			first_frame,
			last_frame,
			fps,
		}
	}
}

impl From<(&str, i8, u16, u16, f32)> for AnimatedTextureConfiguration {
	fn from(t: (&str, i8, u16, u16, f32)) -> Self {
		Self {
			prefix:           t.0.to_owned(),
			number_of_digits: t.1,
			first_frame:      t.2,
			last_frame:       t.3,
			fps:              t.4,
		}
	}
}

#[derive(Debug)]
pub struct EntityConfigurationStateDirection {
	name:     String,
	template: String,
}

impl EntityConfigurationStateDirection {
	pub fn new(name: &str, template: &str) -> Self {
		Self {
			name:     name.to_string(),
			template: template.to_string(),
		}
	}
}

#[derive(Debug)]
pub struct EntityConfigurationState {
	name:       String,
	size:       [f32; 2],
	offset:     [f32; 2],
	directions: HashMap<String, EntityConfigurationStateDirection>,
}

impl EntityConfigurationState {
	pub fn new(name: &str, size: &[f32; 2], offset: &[f32; 2]) -> Self {
		Self {
			name:       name.to_string(),
			size:       size.clone(),
			offset:     offset.clone(),
			directions: HashMap::new(),
		}
	}
	pub fn add_direction(&mut self, direction: EntityConfigurationStateDirection) {
		self.directions.insert(direction.name.clone(), direction);
	}
}

#[derive(Debug)]
pub struct EntityConfiguration {
	name:        String,
	entity_type: String,
	states:      HashMap<String, EntityConfigurationState>,
	//	pub entity_id: EntityId,
	//	pub entity_type: EntityType,
	//	pub animated_texture_configuration: AnimatedTextureConfiguration,
}

impl EntityConfiguration {
	pub fn new(
		name: &str,
		entity_type: &str,
		//		size: Vector2,
		//		animated_texture_configuration: AnimatedTextureConfiguration,
	) -> Self {
		Self {
			name:        name.to_string(),
			entity_type: entity_type.to_string(),
			states:      HashMap::new(),
			//			entity_id: EntityId::NONE,
			//			entity_type: EntityType::None,
			//			size,
			//			animated_texture_configuration,
		}
	}

	pub fn add_state(&mut self, state: EntityConfigurationState) {
		self.states.insert(state.name.clone(), state);
	}
}

// :TEMP: until I know where this will be going

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct EntityConfigurationYamlStateDirection {
	template: String,
}

impl Default for EntityConfigurationYamlStateDirection {
	fn default() -> Self {
		Self {
			template: "[template]".to_string(),
		}
	}
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct EntityConfigurationYamlState {
	first_frame: u16,
	last_frame:  u16,
	size:        [f32; 2],
	offset:      [f32; 2],
	directions:  HashMap<String, EntityConfigurationYamlStateDirection>,
}

impl Default for EntityConfigurationYamlState {
	fn default() -> Self {
		Self {
			first_frame: 0,
			last_frame:  1,
			size:        [32.0, 64.0],
			offset:      [4.0, 8.0],
			directions:  HashMap::new(),
		}
	}
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct EntiyConfigurationYamlEntity {
	name:        String,
	#[serde(rename = "type")]
	entity_type: String,
	states:      HashMap<String, EntityConfigurationYamlState>,
}
impl Default for EntiyConfigurationYamlEntity {
	fn default() -> Self {
		Self {
			name:        "[name]".to_string(),
			entity_type: "[type]".to_string(),
			states:      HashMap::new(),
		}
	}
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct EntiyConfigurationYaml {
	entities: Vec<EntiyConfigurationYamlEntity>,
}
impl Default for EntiyConfigurationYaml {
	fn default() -> Self {
		let mut entities = Vec::new();
		entities.push(EntiyConfigurationYamlEntity::default());
		Self { entities }
	}
}

#[derive(Debug)]
pub struct EntityConfigurationManager {
	configs: HashMap<String, EntityConfiguration>,
}

impl EntityConfigurationManager {
	pub fn new() -> Self {
		Self {
			configs: HashMap::new(),
		}
	}

	fn add_config(&mut self, ec: EntityConfiguration) {
		self.configs.insert(ec.name.clone(), ec);
	}

	pub fn load(&mut self, _system: &mut System, _name: &str) -> bool {
		true
	}

	pub fn load_yaml(&mut self, system: &mut System, filename: &str) -> anyhow::Result<()> {
		// :HACK
		let ec = EntiyConfigurationYamlEntity::default();
		let ecy = serde_yaml::to_string(&ec)?;
		println!("{}", &ecy);

		let yaml = system
			.default_filesystem_mut()
			.open(filename)
			.read_as_string();
		println!("yaml:\n{}", &yaml);
		let ecye: EntiyConfigurationYamlEntity = match serde_yaml::from_str(&yaml) {
			Err(e) => {
				println!("{:#?}", &e);
				anyhow::bail!("{}", &e)
			},
			Ok(d) => d,
		};

		println!("{:?}", ecye);

		let mut ec = EntityConfiguration::new(&ecye.name, &ecye.entity_type);
		for (k, v) in ecye.states {
			let mut s = EntityConfigurationState::new(&k, &v.size, &v.offset);

			for (dk, dv) in v.directions {
				let mut d = EntityConfigurationStateDirection::new(&dk, &dv.template);
				s.add_direction(d);
			}
			ec.add_state(s);
		}

		self.add_config(ec);
		Ok(())
	}

	pub fn get_config(&self, name: &str) -> &EntityConfiguration {
		match self.configs.get(name) {
			Some(ec) => ec,
			None => {
				// return any
				if self.configs.len() == 0 {
					panic!("Tried to get entity configuration without loading!");
				};
				println!("Warning: No configuration found for entity {}", &name);

				self.configs.values().next().unwrap()
			},
		}
	}
}
