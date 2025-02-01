// SPDX-License-Identifier: MPL-2.0
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]
#![allow(clippy::missing_safety_doc)]

pub mod byond;
pub mod dmstring;

use self::{byond::*, dmstring::DMStringWrapper};
use libloading::Library;
use std::{
	cell::RefCell,
	error::Error,
	ffi::{c_char, c_int, CString},
};

#[cfg(not(all(
	target_pointer_width = "32",
	any(target_os = "windows", target_os = "linux")
)))]
compile_error!(
	"byond-memorystats is only for BYOND (not OpenDream), which only supports 32-bit Windows or \
	 Linux!"
);

// byond doesn't free returned strings, so we kinda just re-use the same
// allocation over and over
thread_local! {
	static RETURN_STRING: RefCell<CString> = RefCell::new(CString::default());
}

#[no_mangle]
pub unsafe extern "C" fn memory_stats(_argc: c_int, _argv: *const *const c_char) -> *const c_char {
	let memory_stats_inner = move || -> Result<CString, Box<dyn Error>> {
		let byondcore = Library::new(BYONDCORE)?;
		let dm_string_new = byondcore.get::<DmStringNew>(DMSTRING_NEW_SYMBOL)?;
		let dm_string_free = byondcore.get::<DmStringFree>(DMSTRING_FREE_SYMBOL)?;
		let get_server_mem_usage =
			byondcore.get::<GetServerMemUsage>(GET_SERVER_MEM_USAGE_SYMBOL)?;
		let mut stats = DMStringWrapper::new(&dm_string_new, &dm_string_free);
		get_server_mem_usage(stats.as_mut() as *mut _);
		Ok(CString::from(stats.as_ref()))
	};

	let retval = memory_stats_inner()
		.unwrap_or_else(|err| CString::new(format!("error: {err}")).unwrap_or_default());
	RETURN_STRING.with(|buffer| {
		buffer.replace(retval);
		buffer.borrow().as_ptr()
	})
}
