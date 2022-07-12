pub mod entity;
pub use entity::Entity;
pub use entity::EntityData;
pub use entity::EntityState;
mod entity_configuration;
pub use entity_configuration::AnimatedTextureConfiguration;
pub use entity_configuration::EntityConfiguration;
pub use entity_configuration::EntityConfigurationManager;
mod entity_ids;
pub use entity_ids::EntityId;
mod entity_types;
pub use entity_types::EntityType;
/*
mod entity_manager;
	pub use entity_manager::EntityManager as EntityManager;
	*/
/*
mod background;
	pub use background::Background as Background;
mod coin;
	pub use coin::Coin as Coin;
mod obstacle;
	pub use obstacle::Obstacle as Obstacle;
mod fish;
	pub use fish::Fish as Fish;
*/
mod player;
pub use player::Player;

mod background;
pub use background::Background;

mod entity_manager;
pub use entity_manager::EntityManager;