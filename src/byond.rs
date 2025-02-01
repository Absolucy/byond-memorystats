// SPDX-License-Identifier: MPL-2.0
use crate::dmstring::DMString;

pub type DmStringNew = unsafe extern "thiscall" fn(this: *mut DMString) -> *mut DMString;
pub type DmStringFree = unsafe extern "thiscall" fn(this: *mut DMString);
pub type GetServerMemUsage = unsafe extern "stdcall" fn(out: *mut DMString);

cfg_if::cfg_if! {
	if #[cfg(windows)] {
		pub const BYONDCORE: &str = "byondcore.dll";
		pub const DMSTRING_NEW_SYMBOL: &[u8] = c"??0DMString@@QAE@XZ".to_bytes_with_nul();
		pub const DMSTRING_FREE_SYMBOL: &[u8] = c"??1DMString@@QAE@XZ".to_bytes_with_nul();
		pub const GET_SERVER_MEM_USAGE_SYMBOL: &[u8] =
			c"?GetServerMemUsage@DungServer@@QAE?AUDMString@@XZ".to_bytes_with_nul();
	} else {
		pub const BYONDCORE: &str = "libbyond.so";
		pub const DMSTRING_NEW_SYMBOL: &[u8] = c"_ZN8DMStringC2Ev".to_bytes_with_nul();
		pub const DMSTRING_FREE_SYMBOL: &[u8] = c"_ZN8DMStringD2Ev".to_bytes_with_nul();
		pub const GET_SERVER_MEM_USAGE_SYMBOL: &[u8] =
			c"_ZN10DungServer17GetServerMemUsageEv".to_bytes_with_nul();
	}
}
