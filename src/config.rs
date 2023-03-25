use super::*;

/// Configuration manager.
#[derive(Default)]
pub struct ConfigLoader {
	config: String,
	section: u32,
}

impl cvar::IVisit for ConfigLoader {
	fn visit(&mut self, f: &mut dyn FnMut(&mut dyn cvar::INode)) {
		f(&mut cvar::Action(s!("set!"), |args, _| {
			args.clone_into(&mut self.config);
			// Reload weapon config later
			self.section = 0;
		}));
	}
}

impl ConfigLoader {
	/// Reload the config settins when the section changes.
	#[inline]
	pub fn run(&mut self, api: &mut Api, section: u32, root: &mut dyn cvar::IVisit, ) {
		if section != self.section {
			self.section = section;
			loads(api, &self.config, section, root);
		}
	}

	/// Loads a config string.
	pub fn loads(&mut self, api: &mut Api, config: &str, root: &mut dyn cvar::IVisit, ) {
		self.section = 0;
		config.clone_into(&mut self.config);
		loads(api, &self.config, 0, root);
	}
}

#[inline(never)]
fn loads(api: &mut Api, config: &str, section: u32, root: &mut dyn cvar::IVisit) {
	let mut mask = section == 0;
	for (line, item) in ini_core::Parser::new(config).enumerate() {
		match item {
			ini_core::Item::Section(name) => {
				let name = hash(name);
				if name == hash!("DEFAULT") {
					mask = section == 0;
				}
				else if name == hash!("ANY") {
					mask = section != 0;
				}
				else {
					mask = name == section;
				}
			},
			ini_core::Item::Property(key, value) => {
				if mask {
					let mut writer = String::new();
					if !cvar::console::set(root, key, value, &mut writer) {
						api.log(writer);
					}
				}
			},
			ini_core::Item::Comment(_) => {},
			ini_core::Item::Blank => {},
			ini_core::Item::Action(_) | ini_core::Item::Error(_) => {
				api.log(f!("syntax error at line "{line}));
			},
		}
	}
}
