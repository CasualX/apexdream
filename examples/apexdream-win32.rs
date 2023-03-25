use std::str;
use external::process::{Process, ProcessRights, ProcessList, ProcessId, ProcessInformation};
use external::wndclass::sleep;
use obfstr::obfstr as s;
use fmtools::fmt;
use intptr::IntPtr as Ptr;

fn load_config(rt: &mut Runtime, inst: &mut apexdream::Instance, path: &str) {
	let config = match std::fs::read_to_string(path) {
		Ok(config) => config,
		Err(err) => {
			rt.log(fmt!("load_config: read_to_string "{path}": "{err}));
			return;
		},
	};

	inst.load_config(rt, &config);
}

fn apex_legends(pid: ProcessId, rt: &mut Runtime) -> bool {
	rt.log(fmt!("Apex Legends"));
	let Ok(gd) = std::fs::read_to_string(s!("gamedata.ini")) else {
		rt.log(fmt!("Error reading gamedata.ini"));
		return false;
	};

	rt.process = Process::attach(pid, ProcessRights::ALL_ACCESS).ok();
	let mut inst = apexdream::Instance::default();

	if inst.attach(rt, &gd) {
		load_config(rt, &mut inst, s!("config.ini"));

		while rt.heartbeat() {
			inst.tick(rt);
			rt.tick(&mut inst);
		}
		let signal = rt.signal;
		rt.log(fmt!("SignalExit("{signal}")"));
	}
	else {
		rt.log(fmt!("Error Instance::new"));
	}
	rt.signal == 2
}

#[inline(never)]
fn check_process(pi: &ProcessInformation) -> Option<fn(ProcessId, &mut Runtime) -> bool> {
	let process_name = pi.image_name_wide();

	if process_name == obfstr::obfwide!("r5apex.exe") {
		return Some(apex_legends);
	}

	return None;
}

fn main() {
	let mut signal_exit = false;

	let mut rt = Runtime {
		process: None,
		signal: 0,
	};

	// Track the last attached process id
	// Prevents reattaching to a process that we've deatched from but has yet to exit
	let mut last_process_id = None;
	while !signal_exit {
		// Main loop to find game processes
		{
			let mut seen_last_process_id = false;
			let process_list = ProcessList::query();
			for pi in process_list.iter() {
				let process_id = pi.process_id();
				// Ignore the last process id until it has gone away
				if Some(process_id) == last_process_id {
					seen_last_process_id = true;
					continue;
				}
				// Try to find and attach to known process
				if let Some(run) = check_process(pi) {
					rt.signal = 0;
					if run(process_id, &mut rt) {
						return;
					}
					rt.process = None;
					last_process_id = Some(process_id);
					seen_last_process_id = true;
					break;
				}
			}
			// Clear the last process id if it hasn't been seen
			if !seen_last_process_id {
				last_process_id = None;
			}
		}
		// Handle any client connections
		rt.tick(&mut cvar::Visit(|f| {
			f(&mut cvar::Action(s!("exit!"), |_, _| signal_exit = true));
			f(&mut cvar::Action(s!("retry!"), |_, _| last_process_id = None));
		}));
		// Sleep some time before looking again
		sleep(100);
	}
}

struct Runtime {
	process: Option<external::prelude::Process>,
	signal: u8,
}
impl Runtime {
	fn heartbeat(&self) -> bool {
		if self.signal != 0 {
			return false;
		}
		let Some(process) = &self.process else { return false };
		match process.exit_code() {
			Ok(None) => true,
			_ => false,
		}
	}

	fn tick(&mut self, root: &mut dyn cvar::IVisit) {
		let mut signal_data = 0;
		let signal = &mut signal_data;
		let is_process = self.process.is_some();

		let _visitor = cvar::Visit(move |f| {
			if is_process {
				f(&mut cvar::Action(s!("exit!"), |_, _| *signal = 2));
				f(&mut cvar::Action(s!("break!"), |_, _| *signal = 1));
			}
			root.visit(f);
		});

		if signal_data > 0 {
			self.signal = signal_data;
		}
	}

	fn log(&mut self, args: impl std::fmt::Display) {
		apexdream::Interface::log(self, format_args!("{}", args));
	}
}

fn vm_images(process: &Process) -> impl '_ + Clone + Iterator<Item = Ptr> {
	process.vm_allocations()
		.filter_map(|(address, _, ty)| {
			if ty != external::memory::MemoryType::IMAGE {
				return None;
			}
			Some(address)
		})
}

#[allow(unused_variables)]
impl apexdream::Interface for Runtime {
	fn get_time(&mut self) -> f64 {
		external::system::time_s()
	}

	fn sleep(&mut self, ms: u32) {
		external::prelude::sleep(ms);
	}

	fn log(&mut self, args: std::fmt::Arguments) {
		println!("{}", args);
	}

	fn visualize(&mut self, scope: &str, args: std::fmt::Arguments) {
		// todo!()
	}

	fn dump_bin(&mut self, path: &str, data: &[u8]) {
		match std::fs::write(path, data) {
			Ok(_) => (),
			Err(err) => self.log(fmt!("Failed to write "{path}": "{err})),
		}
	}

	fn gamestate_has_listeners(&mut self) -> bool {
		false
	}

	fn gamestate_update(&mut self, _scope: &str, _state: &str) {
		return;
	}

	fn mouse_move(&mut self, dx: i32, dy: i32) {
		external::mouse::Mouse.mouse_move(dx, dy);
	}

	fn base_address(&mut self) -> u64 {
		let Some(process) = &self.process else { return 0u64 };
		let mut base_address = 0;
		obfstr::obfwide! {
			let dot_exe = ".exe";
		}
		let mut buffer = [0; 260];
		for address in vm_images(process) {
			if let Ok(file_name) = process.get_mapped_file_name_wide(address, &mut buffer) {
				if file_name.ends_with(dot_exe) {
					base_address = address.into_raw();
				}
			}
		}
		base_address as u64
	}

	fn read_memory(&mut self, address: u64, dest: &mut [u8]) -> i32 {
		let Some(process) = &self.process else { return -1 };
		match process.vm_read_into(external::prelude::IntPtr::from_usize(address as usize), dest) {
			Ok(_) => 0,
			Err(_) => -1,
		}
	}

	// Pasted from some other project, I hope it works :)
	// Can probably be implemented a little smarter on Win32
	fn gather_memory(&mut self, base_address: u64, size: u32, indices: &mut [u32]) -> i32 {
		let mut buf = [0u8; 0x1000];

		// Keep track of indices read within reasonable limit
		if indices.len() >= 128 {
			return -1;
		}
		let mut read_mask = 0u128;

		// For every index
		let mut success = false;
		for i in 0..indices.len() {
			if read_mask & (1u128 << i) == 0 {
				let virtual_address = (base_address + indices[i] as u64) & !0xfff;
				let temp = if self.read_memory(virtual_address, &mut buf) >= 0 {
					// If a single read was succesful the whole read is successful
					success = true;
					Some(&buf)
				}
				else {
					None
				};

				// Read all indices in the page
				for j in i..indices.len() {
					if read_mask & (1u128 << j) == 0 {
						let index_address = base_address + indices[j] as u64;
						if index_address >= virtual_address && index_address < virtual_address + 0x1000 {
							// Mark the index as read
							read_mask |= 1u128 << j;

							// Try to read the index
							// Write zero if underlying page failed to read or index straddling 4K boundary
							let index_offset = (index_address - virtual_address) as usize;
							indices[j] = temp
								.and_then(|temp| temp.get(index_offset..index_offset + 4))
								.map(|dword| u32::from_ne_bytes([dword[0], dword[1], dword[2], dword[3]]))
								.unwrap_or(0);
						}
					}
				}
			}
		}

		if success { 0 } else { -1 }
	}

	fn write_memory(&mut self, address: u64, src: &[u8]) -> i32 {
		let Some(process) = &self.process else { return -1 };
		match process.vm_write(external::prelude::IntPtr::from_usize(address as usize), src) {
			Ok(_) => 0,
			Err(_) => -1,
		}
	}

	// Overlay rendering currently not implemented!
	fn r_begin(&mut self, screen: &mut [i32; 2]) -> bool { false }
	fn r_rect(&mut self, x: f32, y: f32, width: f32, height: f32, fill: u32, stroke: u32) {}
	fn r_ellipse(&mut self, x: f32, y: f32, width: f32, height: f32, fill: u32, stroke: u32) {}
	fn r_text(&mut self, font: u32, flags: u32, x: f32, y: f32, width: f32, height: f32, color: u32, color2: u32, text: &str) {}
	fn r_line(&mut self, color: u32, x1: f32, y1: f32, x2: f32, y2: f32) {}
	fn r_lines(&mut self, color: u32, points: &[[f32; 2]], lines: &[[u16; 2]]) {}
	fn r_image(&mut self, image: u32, sx: f32, sy: f32, swidth: f32, sheight: f32, dx: f32, dy: f32, dwidth: f32, dheight: f32, opacity: f32) {}
	fn r_end(&mut self) {}
}
