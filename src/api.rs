use std::{fmt, mem, ops};
use intptr::IntPtr64 as Ptr;
use dataview::Pod;
use crate::Interface;

pub struct Error;

#[repr(transparent)]
pub struct Api {
	i: dyn Interface,
}

#[inline]
#[allow(non_snake_case)]
pub fn Api(this: &mut dyn Interface ) -> &mut Api {
	unsafe { mem::transmute(this) }
}

impl ops::Deref for Api {
	type Target = dyn Interface;
	#[inline]
	fn deref(&self) -> &(dyn Interface + 'static) {
		&self.i
	}
}
impl ops::DerefMut for Api {
	#[inline]
	fn deref_mut(&mut self) -> &mut (dyn Interface + 'static) {
		&mut self.i
	}
}

#[inline(never)]
fn log(this: &mut Api, args: &dyn fmt::Display) {
	(**this).log(format_args!("{}", args))
}
#[inline(never)]
fn visualize(this: &mut Api, scope: &str, args: &dyn fmt::Display) {
	(**this).visualize(scope, format_args!("{}", args))
}

impl Api {
	/// Standard log function.
	pub fn log(&mut self, args: impl fmt::Display) {
		log(self, &args)
	}

	/// Visualize the args in a scope.
	///
	/// The `args` is some html that should replace the previous contents of `scope`.
	pub fn visualize(&mut self, scope: &str, args: impl fmt::Display) {
		visualize(self, scope, &args)
	}

	/// Reads memory from the process.
	#[cfg_attr(feature = "debug_api", track_caller)]
	#[inline]
	pub fn vm_read<T: Pod>(&mut self, ptr: Ptr<T>) -> Result<T, Error> {
		unsafe {
			// Yes yes but this isn't easy to fix...
			#[allow(deprecated)]
			let mut dest: T = mem::uninitialized();
			let result = {
				let dest = dataview::bytes_mut(&mut dest);
				self.i.read_memory(ptr.into_raw(), dest)
			};
			if result >= 0 { Ok(dest) }
			else {
				#[cfg(feature = "debug_api")]
				self.log(fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_read("{ptr}"): "{result}));
				Err(Error)
			}
		}
	}

	/// Reads memory into the destination from the process.
	#[cfg_attr(feature = "debug_api", track_caller)]
	#[inline]
	pub fn vm_read_into<T: Pod + ?Sized>(&mut self, ptr: Ptr<T>, dest: &mut T) -> Result<(), Error> {
		let result = {
			let dest = dataview::bytes_mut(dest);
			self.i.read_memory(ptr.into_raw(), dest)
		};
		if result >= 0 { Ok(()) }
		else {
			#[cfg(feature = "debug_api")]
			self.log(fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_read_into("{ptr}"): "{result}));
			Err(Error)
		}
	}

	/// Gathers memory from the process.
	/// This routine is optimized for reading small pieces of large objects.
	#[cfg_attr(feature = "debug_api", track_caller)]
	#[inline]
	pub fn vm_gatherd<'a, T: Pod>(&mut self, ptr: Ptr, size: u32, indices: &'a mut T) -> Result<&'a T, Error> {
		let view_mut = dataview::DataView::from_mut(indices);
		let view_mut = view_mut.slice_mut::<u32>(0, view_mut.tail_len::<u32>(0));
		let result = self.gather_memory(ptr.into_raw(), size, view_mut);
		if result >= 0 { Ok(indices) }
		else {
			#[cfg(feature = "debug_api")]
			self.log(fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_gatherd("{ptr}"): "{result}));
			Err(Error)
		}
	}

	/// Reads bytes to be interpreted as a c-string.
	pub fn vm_read_cstr<'a>(&mut self, ptr: Ptr<[u8]>, buf: &'a mut [u8]) -> Result<&'a str, Error> {
		self.vm_read_into(ptr, buf)?;
		crate::base::from_utf8_buf(buf).ok_or(Error)
	}

	/// Writes memory into the process.
	#[cfg_attr(feature = "debug_api", track_caller)]
	#[inline]
	pub fn vm_write<T: Pod + ?Sized>(&mut self, ptr: Ptr<T>, data: &T) -> Result<(), Error> {
		let result = {
			let data = dataview::bytes(data);
			self.write_memory(ptr.into_raw(), data)
		};
		if result >= 0 { Ok(()) }
		else {
			#[cfg(feature = "debug_api")]
			self.log(fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_write("{ptr}"): "{result}));
			Err(Error)
		}
	}

	/// Draws a rectangle.
	///
	/// If `fill` is non-zero then the rectangle should be filled with this ARGB color.
	/// If `stroke` is non-zero then the rectangle should be stroked with this ARGB color.
	#[inline]
	pub fn r_rect(&mut self, x: f32, y: f32, width: f32, height: f32, fill: vgc::sRGBA, stroke: vgc::sRGBA) {
		self.i.r_rect(x, y, width, height, fill.pack(), stroke.pack())
	}

	/// Draws an ellipse.
	///
	/// The ellipse is defined by the rectangle.
	///
	/// If `fill` is non-zero then the rectangle should be filled with this ARGB color.
	/// If `stroke` is non-zero then the rectangle should be stroked with this ARGB color.
	#[inline]
	pub fn r_ellipse(&mut self, x: f32, y: f32, width: f32, height: f32, fill: vgc::sRGBA, stroke: vgc::sRGBA) {
		self.i.r_ellipse(x, y, width, height, fill.pack(), stroke.pack())
	}

	/// Draws text.
	///
	/// TODO! Write detailed explanation.
	#[inline]
	pub fn r_text(&mut self, font: u32, flags: u32, x: f32, y: f32, width: f32, height: f32, color: vgc::sRGBA, color2: vgc::sRGBA, text: &str) {
		self.i.r_text(font, flags, x, y, width, height, color.pack(), color2.pack(), text)
	}

	/// Draws a line.
	#[inline]
	pub fn r_line(&mut self, color: vgc::sRGBA, x1: f32, y1: f32, x2: f32, y2: f32) {
		self.i.r_line(color.pack(), x1, y1, x2, y2)
	}

	/// Draws indexed lines.
	#[inline]
	pub fn r_lines(&mut self, color: vgc::sRGBA, points: &[[f32; 2]], lines: &[[u16; 2]]) {
		self.i.r_lines(color.pack(), points, lines)
	}

	/// Draws a subrectangle of an image.
	#[inline]
	pub fn r_image(&mut self, image: u32, sx: f32, sy: f32, swidth: f32, sheight: f32, dx: f32, dy: f32, dwidth: f32, dheight: f32, opacity: f32) {
		self.i.r_image(image, sx, sy, swidth, sheight, dx, dy, dwidth, dheight, opacity)
	}
}
