/*!

Analyzes CInput::GetButtonBits to extract the global button input state and their values.

From the Source SDK 2013: https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/client/in_main.cpp#L1447

 */

use format_xml::template;
use pelite::Pod;
use pelite::pattern as pat;
use pelite::pe64::*;

pub fn print(bin: PeFile<'_>, dll_name: &str) {
	let btns = buttons(bin);

	template::print! {
		"## Buttons\n\n"
		"These are addresses to global instances of the [`kbutton_t`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/client/kbutton.h#L14-L20) struct.\n\n"
		"```\n"
		for btn in (&btns) {
			{dll_name}"!"{btn.kbutton;#010x}" kbutton_t in_"{&btn.name[1..]}"\n"
		}
		"```\n\n"
	}
}

#[derive(Copy, Clone, Pod, Debug)]
#[repr(C)]
struct kbutton_t {
	down: [i32; 2],
	state: i32,
}

struct Button<'a> {
	name: &'a str,
	kbutton: u32,
}

fn buttons<'a>(file: PeFile<'a>) -> Vec<Button<'a>> {
	let mut save = [0; 4];
	let mut btns = Vec::new();

	// Match the ConCommand in .data section...
	let data_section = file.section_headers().iter().find(|sect| &sect.Name == b".data\0\0\0").unwrap();
	let pat = pat!("@3 [8] 0100000000000000 *{'2B} [48] *4053 [20-60] 8B05${'} 3BD8 74?");
	let mut matches = file.scanner().matches(pat, data_section.virtual_range());
	while matches.next(&mut save) {
		let name = file.derva_c_str(save[1]).unwrap().to_str().unwrap();
		let kbutton = save[2];
		btns.push(Button { name, kbutton });
	}

	// Sort the list by name for improved diff viewer experience
	btns.sort_unstable_by_key(|cmd| cmd.name);
	return btns;
}
