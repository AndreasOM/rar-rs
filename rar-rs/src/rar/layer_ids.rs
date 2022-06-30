#[derive(Debug, Copy, Clone)]
pub enum LayerId {
	None          = 0,
	Background    = 1,
	Player		  = 5,
	Debug         = 7,
	Ui            = 8,
	DebugRenderer = 11,
}
