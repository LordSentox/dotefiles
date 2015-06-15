/*
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

use dote::{Script};

use std::path::Path;
use std::fs::File;
use std::io;

impl Script {
	pub fn open (name: &str) -> Result <Script, io::Error> {
		let mut path_string = name.to_string();
		path_string.push_str(".dote");
		let path_string = path_string;

		let path = Path::new(&path_string);
		let mut file = match File::open(&path) {
			Ok (file) => file,
			Err (code) => return Err (code)
		};

		Ok (
			Script {
				path: name.to_string() + ".dote",
				file: file
			}
		)
	}

	pub fn path (&self) -> &String {
		&self.path
	}
}
