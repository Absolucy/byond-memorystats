// SPDX-License-Identifier: MPL-2.0
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]

pub mod byond;
pub mod dmstring;

use self::dmstring::DMString;
use std::{
	cell::RefCell,
	ffi::{c_char, c_int, CString},
};

#[cfg(not(all(
	target_pointer_width = "32",
	any(target_os = "windows", target_os = "linux")
)))]
compile_error!(
	"byond-memorystats is only for BYOND (not OpenDream), which only supports on 32-bit Windows \
	 or Linux!"
);

// byond doesn't free returned strings, so we kinda just re-use the same
// allocation over and over
thread_local! {
	static RETURN_STRING: RefCell<CString> = RefCell::new(CString::default());
}

#[no_mangle]
pub extern "C" fn memory_stats(_argc: c_int, _argv: *const *const c_char) -> *const c_char {
	let mut stats = DMString::default();
	unsafe { byond::get_server_mem_usage(&mut stats) };
	let stats = CString::from(stats.as_ref());
	RETURN_STRING.with(|buffer| {
		buffer.replace(stats);
		buffer.borrow().as_ptr()
	})
}
