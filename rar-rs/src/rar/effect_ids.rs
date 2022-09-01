/*
#[derive(Debug, Copy, Clone)]
pub enum EffectId {
	None                = 0,
	Default             = 1,
	White               = 2,
	Colored             = 3,
	Textured            = 4,
	ColoredTextured     = 5,
	Background          = 6,
	FontColored         = 7,
	TexturedDesaturated = 8,
}
*/


pub use oml_game::renderer::EffectId;
pub use oml_game::renderer::EffectIdFactory;
/*
pub struct EffectIdNone {}
impl EffectId for EffectIdNone {
	fn as_u16() -> u16 { 0 }
}

pub struct EffectIdColored {}
impl EffectId for EffectIdColored {
	fn as_u16() -> u16 { 3 }
}

pub struct EffectIdTextured {}
impl EffectId for EffectIdTextured {
	fn as_u16() -> u16 { 4 }
}

pub struct EffectIdBackground {}
impl EffectId for EffectIdBackground {
	fn as_u16() -> u16 { 6 }
}

pub struct EffectIdTexturedDesaturated {}
impl EffectId for EffectIdTexturedDesaturated {
	fn as_u16() -> u16 { 8 }
}
*/

macro_rules! generate_effect_id{
	($a:ident,$b:expr)=>{
		pub struct $a {}
		impl EffectId for $a {
			fn as_u16() -> u16 { $b }
		}
	}
}


generate_effect_id!(EffectIdNone,0);
generate_effect_id!(EffectIdColored,3);
generate_effect_id!(EffectIdTextured,4);
generate_effect_id!(EffectIdBackground,6);
generate_effect_id!(EffectIdTexturedDesaturated,8);


pub struct RarEffectIdFactory {}
impl EffectIdFactory for RarEffectIdFactory {
	fn create( name: &str ) -> Box<dyn EffectId> {
		match name {
			"Colored" => Box::new(EffectIdColored {}),
			"Textured" => Box::new(EffectIdTextured {}),
			"Background" => Box::new(EffectIdBackground {}),
			"TexturedDesaturated" => Box::new(EffectIdTexturedDesaturated {}),
			_ => Box::new(EffectIdNone {}),
		}
	}

}
enum EffectIds {
	None                = 0,
	Default             = 1,
	White               = 2,
	Colored             = 3,
	Textured            = 4,
	ColoredTextured     = 5,
	Background          = 6,
	FontColored         = 7,
	TexturedDesaturated = 8,
}
/*
macro_rules! generate_effect_ids_from_enum{
	($a:enum) => {

	}
}

generate_effect_ids_from_enum!( EffectIds );
*/
