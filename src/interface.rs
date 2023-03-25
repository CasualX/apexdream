use std::fmt;

/// Cheat interface.
pub trait Interface {
	/// Returns a timestamp in seconds.
	///
	/// Must be a high precision timestamp, strictly increasing values.
	fn get_time(&mut self) -> f64;

	/// Standard sleep, may be ignored.
	fn sleep(&mut self, ms: u32);

	/// Standard log function.
	fn log(&mut self, args: fmt::Arguments);

	/// Visualize the args in a scope.
	///
	/// The `args` is some html that should replace the previous contents of `scope`.
	fn visualize(&mut self, scope: &str, args: fmt::Arguments);

	/// Write the dumped game binary to disk.
	fn dump_bin(&mut self, path: &str, data: &[u8]);

	/// Moves the mouse relatively by the given deltas.
	fn mouse_move(&mut self, dx: i32, dy: i32);

	/// Returns the base address of the game process' executable.
	fn base_address(&mut self) -> u64;

	/// Reads memory from the game process.
	///
	/// Negative return value indicates an error.
	fn read_memory(&mut self, address: u64, dest: &mut [u8]) -> i32;

	/// Gathers memory from the game process.
	///
	/// This routine is optimized for reading small pieces of large objects.
	/// Negative return value indicates an error.
	fn gather_memory(&mut self, address: u64, size: u32, indices: &mut [u32]) -> i32;

	/// Writes memory into the game process.
	///
	/// Negative return value indicates an error.
	fn write_memory(&mut self, address: u64, src: &[u8]) -> i32;

	/// Begins rendering the overlay.
	///
	/// Returns `false` if drawing should be skipped.
	///
	/// Output the screen width and height in pixels in the `screen` value.
	fn r_begin(&mut self, screen: &mut [i32; 2]) -> bool;

	/// Draws a rectangle.
	///
	/// If `fill` is non-zero then the rectangle should be filled with this ARGB color.
	/// If `stroke` is non-zero then the rectangle should be stroked with this ARGB color.
	fn r_rect(&mut self, x: f32, y: f32, width: f32, height: f32, fill: u32, stroke: u32);

	/// Draws an ellipse.
	///
	/// The ellipse is defined by the rectangle.
	///
	/// If `fill` is non-zero then the rectangle should be filled with this ARGB color.
	/// If `stroke` is non-zero then the rectangle should be stroked with this ARGB color.
	fn r_ellipse(&mut self, x: f32, y: f32, width: f32, height: f32, fill: u32, stroke: u32);

	/// Draws text.
	///
	/// TODO! Write detailed explanation.
	fn r_text(&mut self, font: u32, flags: u32, x: f32, y: f32, width: f32, height: f32, color: u32, color2: u32, text: &str);

	/// Draws a line.
	fn r_line(&mut self, color: u32, x1: f32, y1: f32, x2: f32, y2: f32);

	/// Draws indexed lines.
	fn r_lines(&mut self, color: u32, points: &[[f32; 2]], lines: &[[u16; 2]]);

	/// Draws a subrectangle of an image.
	fn r_image(&mut self, image: u32, sx: f32, sy: f32, swidth: f32, sheight: f32, dx: f32, dy: f32, dwidth: f32, dheight: f32, opacity: f32);

	/// Ends rendering the overlay.
	fn r_end(&mut self);
}
