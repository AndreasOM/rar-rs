#[derive(Debug, Clone, PartialEq)]
pub enum AudioMessage {
	PlaySound(String),
	ToggleMusic,
}
