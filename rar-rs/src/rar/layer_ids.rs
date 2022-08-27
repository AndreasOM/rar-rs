#[derive(Debug, Copy, Clone)]
pub enum LayerId {
	None          = 0,
	Background    = 1,
	TileMap1      = 2,
	TileMap2      = 3,
	Player        = 5,
	Debug         = 7,
	Ui            = 8,
	DebugRenderer = 11,
}
