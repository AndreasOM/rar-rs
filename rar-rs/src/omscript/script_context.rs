pub trait ScriptContext {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "[ScriptContext]")
	}
}

impl core::fmt::Debug for dyn ScriptContext {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		self.fmt(f)
	}
}
