use super::*;

#[derive(Default)]
pub struct GameProcess {
    pub base: sdk::Ptr,
    attached_time: f64,
    logged_is_valid: bool,
    is_ready: bool,
    pub time_date_stamp: u32,
    pub checksum: u32,
    pub size_of_image: u32,

    dump_game: bool,
    dump_fast: bool,
    dump_cancel: bool,
    dump_rate: f32,
    dump_start: f64,
    dump_timer: base::Timer,
    image_data: Box<[u8]>,
    page_index: u32,
    read_errors: u32,
    gadgets: [u32; 2],
}

impl GameProcess {
    pub fn attach(&mut self, api: &mut Api) -> bool {
        self.attached_time = 0.0;
        self.logged_is_valid = false;
        self.is_ready = false;
        self.time_date_stamp = 0;
        self.checksum = 0;
        self.size_of_image = 0;

        let base = sdk::Ptr::from_raw(api.base_address());
        api.log(f!({base:#x}" r5apex.exe"));

        self.base = base;
        if base.is_null() {
            return false;
        }

        {
            use pelite::pe64::*;

            let Ok(dos_header) = api.vm_read::<image::IMAGE_DOS_HEADER>(base.cast()) else {
                return false;
            };
            let Ok(nt_headers) =
                api.vm_read::<image::IMAGE_NT_HEADERS>(base.field(dos_header.e_lfanew))
            else {
                return false;
            };

            self.time_date_stamp = nt_headers.FileHeader.TimeDateStamp;
            self.checksum = nt_headers.OptionalHeader.CheckSum;
            self.size_of_image = nt_headers.OptionalHeader.SizeOfImage;

            api.log(f!("TimeDateStamp="{self.time_date_stamp:#x}));
            api.log(f!("CheckSum="{self.checksum:#x}));
            api.log(f!("SizeOfImage="{self.size_of_image:#x}));
        }

        self.attached_time = api.get_time();
        return true;
    }

    /// Returns if the process base address is non-null.
    pub fn is_valid(&mut self, api: &mut Api) -> bool {
        if self.base.is_null() {
            if !self.logged_is_valid {
                self.logged_is_valid = true;
                api.log(f!("Process base address is null!"));
            }
            return false;
        }
        if self.time_date_stamp == 0 || self.size_of_image == 0 {
            if !self.logged_is_valid {
                self.logged_is_valid = true;
                api.log(f!("TimeDateStamp is zero!"));
            }
            return false;
        }
        return true;
    }

    /// Checks if the process is ready for reading.
    ///
    /// The game executable is obfuscated, this returns `true` when the game finished deobfuscating itself.
    /// This avoids a period of reading garbage and having to deal with that.
    ///
    /// FIXME! With Hyperion this code will need to be reconsidered.
    pub fn is_ready(&mut self, api: &mut Api, time: f64) -> bool {
        // Once ready always ready
        if self.is_ready {
            return self.is_ready;
        }

        if self.attached_time == 0.0 {
            self.attached_time = time;
        }

        // Wait at least 5sec
        if time < self.attached_time + 5.0 {
            return false;
        }

        // Wait at most 10sec
        // The code below can fail in annoying ways...
        if time >= self.attached_time + 10.0 {
            self.is_ready = true;
            return self.is_ready;
        }

        use pelite::pe64::*;
        use std::mem::size_of;

        let base = self.base;

        let mut dos_header: image::IMAGE_DOS_HEADER = dataview::zeroed();
        if api.vm_read_into(base.cast(), &mut dos_header).is_err() {
            return false;
        }

        let mut nt_headers: image::IMAGE_NT_HEADERS = dataview::zeroed();
        if api
            .vm_read_into(base.field(dos_header.e_lfanew), &mut nt_headers)
            .is_err()
        {
            return false;
        }

        let mut text_section: image::IMAGE_SECTION_HEADER = dataview::zeroed();
        let section_headers = dos_header.e_lfanew
            + size_of::<u32>() as u32
            + size_of::<image::IMAGE_FILE_HEADER>() as u32
            + nt_headers.FileHeader.SizeOfOptionalHeader as u32;
        if api
            .vm_read_into(base.field(section_headers), &mut text_section)
            .is_err()
        {
            return false;
        }

        let mut bytes = [0u8; 8];
        if api
            .vm_read_into(
                base.field(text_section.VirtualAddress + text_section.SizeOfRawData - 8),
                &mut bytes,
            )
            .is_err()
        {
            // If we failed to read this then it may just be paged out.
            // In that case the game has probably finished deobfuscating already.
            self.is_ready = true;
            return self.is_ready;
        }

        // If padding is less than 8 bytes only check last byte
        let n_pad = text_section.SizeOfRawData as i32 - text_section.VirtualSize as i32;
        if n_pad > 0 {
            if if n_pad >= 8 {
                bytes != [0u8; 8]
            } else {
                bytes[7] != 0
            } {
                return false;
            }
        }

        self.is_ready = true;
        return self.is_ready;
    }

    /// Dumps the game executable over time.
    pub fn read_pages(&mut self, api: &mut Api, time: f64) {
        // Dumping the game must happen on-demand
        if !self.dump_game {
            return;
        }

        // Allocate memory for the decrypted image
        if self.image_data.len() != self.size_of_image as usize {
            self.dump_start = time;
            self.image_data = vec![0u8; self.size_of_image as usize].into_boxed_slice();
        }

        // Find the necessary gadget
        if self.gadgets[0] == 0 || self.gadgets[1] == 0 {
            let _ = api.vm_read_into(self.base.cast(), &mut self.image_data[..]);
            if self.dump_fast {
                self.dump_bin(api);
                self.dump_game = false;
                self.image_data = Box::default();
                return;
            } else if !self.find_gadget(api) {
                self.dump_game = false;
                self.image_data = Box::default();
                return;
            }
        }

        if self.dump_rate <= 1.0 {
            api.log(f!("Cannot read_pages with invalid drate!"));
            self.dump_game = false;
            return;
        }

        // Rate limit dumping to ensure the game touches the memory
        let interval = if time < self.dump_start + 5.0 {
            0.2
        } else {
            1.0 / self.dump_rate as f64
        };
        if !self.dump_timer.has_elapsed(time, interval) {
            return;
        }

        let offset = (self.page_index * 0x1000) as usize;
        let Some(image_page) = self.image_data.get_mut(offset..offset + 0x1000) else {
            return;
        };
        if api
            .vm_read_into(self.base.field(offset as u32), image_page)
            .is_err()
        {
            // Reading can fail in which case we just need to wait until the game touches the page
            self.read_errors += 1;
            let _ = api.vm_write(
                self.base.field(self.gadgets[0] + 0x48),
                &self.base.field::<u8>(self.page_index * 0x1000),
            );
            return;
        }

        self.page_index += 1;
        let num_pages = self.size_of_image / 0x1000;

        if self.dump_cancel {
            self.page_index = num_pages;
        }

        // Progress reporting
        api.visualize(s!("ReadPages"), xfmt! {
			<pre>
				"pages:  "{self.page_index}"/"{num_pages}" ("{(self.page_index as f64 / num_pages as f64) * 100.0:.1}"%)\n"
				"errors: "{self.read_errors}"\n"
			</pre>
		});
        if self.page_index % (num_pages / 32).max(1) == 0 || self.page_index == num_pages {
            api.log(f!("ReadPages: "{self.page_index}"/"{num_pages}" ("{(self.page_index as f64 / num_pages as f64) * 100.0:.1}"%)"));
        }

        if self.page_index == num_pages {
            self.dump_bin(api);
            // Restore the gadgets
            let _ = api.vm_write(
                self.base.field(self.gadgets[0] + 0x48),
                &self.base.field::<u8>(self.gadgets[0]),
            );
            let _ = api.vm_write(
                self.base.field(self.gadgets[1] + 0x48),
                &self.base.field::<u8>(self.gadgets[1]),
            );
            // Free memory
            self.dump_game = false;
            self.image_data = Box::default();
        } else {
            // Update the gadgets for the next page
            let _ = api.vm_write(
                self.base.field(self.gadgets[0] + 0x48),
                &self.base.field::<u8>(self.page_index * 0x1000),
            );
            let _ = api.vm_write(
                self.base.field(self.gadgets[1] + 0x48),
                &self
                    .base
                    .field::<u8>(u32::min(self.page_index + 1, num_pages - 1) * 0x1000),
            );
        }
    }

    fn find_gadget(&mut self, api: &mut Api) -> bool {
        obfstr::obfbytes! {
            let cl_showpos = b"cl_showpos\0";
            let net_graph = b"cl_showfps\0";
            let fps_max = b"fps_max\0";
        };

        let image = dataview::DataView::from(&self.image_data[..]);

        let mut i = 0;
        while i < self.size_of_image {
            let rva = i;
            i += 8;

            // Read every pointer sized value
            let Some(ptr) = image.try_read::<sdk::Ptr>(rva as usize) else {
                continue;
            };
            let offset = ptr.into_raw().wrapping_sub(self.base.into_raw());
            if offset >= self.size_of_image as u64 {
                continue;
            }

            // Find the first gadget
            if image.try_get::<[u8; 11]>(offset as usize) == Some(cl_showpos) {
                let gadget_rva = rva - 3 * 8;
                api.log(f!("Found gadget cl_showpos at r5apex!"{gadget_rva:#x}));

                // Double check the gadget before using it
                if image.try_read::<u64>((rva + 6 * 8) as usize)
                    != Some(self.base.field::<()>(gadget_rva).into_raw())
                {
                    api.log(f!("Gadget cl_showpos failed check!"));
                    continue;
                }

                self.gadgets[0] = gadget_rva;
            }
            // Find the second gadget
            else if image.try_get::<[u8; 11]>(offset as usize) == Some(net_graph) {
                let gadget_rva = rva - 3 * 8;
                api.log(f!("Found gadget net_graph at r5apex!"{gadget_rva:#x}));

                // Double check the gadget before using it
                if image.try_read::<u64>((rva + 6 * 8) as usize)
                    != Some(self.base.field::<()>(gadget_rva).into_raw())
                {
                    api.log(f!("Gadget net_graph failed check!"));
                    continue;
                }

                self.gadgets[1] = gadget_rva;
            }
            // Find fps_max
            else if image.try_get::<[u8; 8]>(offset as usize) == Some(fps_max) {
                let fps_max = rva - 3 * 8;
                api.log(f!("Found ConVar fps_max at r5apex!"{fps_max:#x}));

                // Double check the gadget before using it
                if image.try_read::<u64>((rva + 6 * 8) as usize)
                    != Some(self.base.field::<()>(fps_max).into_raw())
                {
                    api.log(f!("ConVar fps_max failed check!"));
                    continue;
                }

                // Set fps_max to 300
                let _ = api.vm_write(self.base.field(fps_max + 0x68), &300.0_f32);
                let _ = api.vm_write(self.base.field(fps_max + 0x6C), &300_i32);
                api.log(f!("ConVar fps_max changed to 300!"));
            }
        }

        return self.gadgets[0] != 0 && self.gadgets[1] != 0;
    }

    fn dump_bin(&mut self, api: &mut Api) {
        let target = &mut self.image_data[..];

        // Fixup PE headers
        let (_, _, data_directory, section_headers) = unsafe { pelite::pe64::headers_mut(target) };
        for section in section_headers {
            section.PointerToRawData = section.VirtualAddress;
            section.SizeOfRawData = section.VirtualSize;
            if &section.Name == b".reloc\0\0" {
                if let Some(reloc_dir) =
                    data_directory.get_mut(pelite::image::IMAGE_DIRECTORY_ENTRY_BASERELOC)
                {
                    reloc_dir.VirtualAddress = section.VirtualAddress;
                    reloc_dir.Size = section.VirtualSize;
                }
            }
        }

        // Finally write the dump to disk
        s! { let path = "/re/apex/r5apex.bin"; };
        api.dump_bin(path, target);
        api.log(f!("Wrote "{path}"!"));
    }
}
