pub trait ScriptContext {
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		write!(f, "[ScriptContext]")
	}
}

impl core::fmt::Debug for dyn ScriptContext {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		self.fmt(f)
	}
}

impl dyn ScriptContext + '_ {
	pub fn script_context_as_then<E: 'static>(&self, f: &dyn Fn(&E)) {
		match self.as_any().downcast_ref::<E>() {
			Some(e) => {
				f(e);
			},
			None => panic!(), //panic!("{:?} isn't a {:?}!", &self, std::any::type_name::<E>(),),
		}
	}
	pub fn script_context_as_mut_then<E: 'static>(&mut self, f: &dyn Fn(&mut E)) {
		match self.as_any_mut().downcast_mut::<E>() {
			Some(e) => {
				f(e);
			},
			None => panic!(), //panic!("{:?} isn't a {:?}!", &self, std::any::type_name::<E>(),),
		}
	}
}
