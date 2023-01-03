#[derive(Debug)]
pub struct UiUpdateContextHelper {}
impl UiUpdateContextHelper {
	fn mut_as_then<E: 'static>(s: &mut dyn UiUpdateContext, f: &mut dyn FnMut(&mut E)) {
		match s.as_any_mut().downcast_mut::<E>() {
			Some(e) => {
				f(e);
			},
			None => {
				let t = ""; // :( // format!("{:?}", &s);
				panic!("{} isn't a {:?}!", t, std::any::type_name::<E>())
			},
		}
	}
}
pub trait UiUpdateContext {
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		writeln!(f, "[Trait] UiUpdateContext")
	}
}

impl std::fmt::Debug for dyn UiUpdateContext {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		self.fmt(f)
	}
}
