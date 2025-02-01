// SPDX-License-Identifier: MPL-2.0
use crate::byond::{DmStringFree, DmStringNew};
use libloading::Symbol;
use std::{
	ffi::{c_char, CStr},
	mem::MaybeUninit,
};

#[repr(C)]
pub struct DMString {
	data: *mut c_char,
}

impl AsRef<CStr> for DMStringWrapper<'_> {
	fn as_ref(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.inner.data) }
	}
}

pub struct DMStringWrapper<'a> {
	inner: DMString,
	free_sym: &'a Symbol<'a, DmStringFree>,
}

impl<'a> DMStringWrapper<'a> {
	pub fn new(new_sym: &Symbol<'_, DmStringNew>, free_sym: &'a Symbol<'a, DmStringFree>) -> Self {
		let mut value = MaybeUninit::uninit();
		unsafe { (new_sym)(value.as_mut_ptr()) };
		Self {
			inner: unsafe { value.assume_init() },
			free_sym,
		}
	}
}

impl Drop for DMStringWrapper<'_> {
	fn drop(&mut self) {
		unsafe {
			(self.free_sym)(&mut self.inner);
		}
	}
}

impl AsRef<DMString> for DMStringWrapper<'_> {
	fn as_ref(&self) -> &DMString {
		&self.inner
	}
}

impl AsMut<DMString> for DMStringWrapper<'_> {
	fn as_mut(&mut self) -> &mut DMString {
		&mut self.inner
	}
}
