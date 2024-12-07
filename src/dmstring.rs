// SPDX-License-Identifier: MPL-2.0
use std::{
	ffi::{c_char, CStr},
	mem::MaybeUninit,
};

#[repr(C)]
pub struct DMString {
	data: *mut c_char,
}

impl Drop for DMString {
	fn drop(&mut self) {
		unsafe {
			crate::byond::dm_string_free(self);
		}
	}
}

impl Default for DMString {
	fn default() -> Self {
		unsafe {
			let mut value = MaybeUninit::uninit();
			crate::byond::dm_string_new(value.as_mut_ptr());
			value.assume_init()
		}
	}
}

impl AsRef<CStr> for DMString {
	fn as_ref(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.data) }
	}
}
