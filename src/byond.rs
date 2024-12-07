// SPDX-License-Identifier: MPL-2.0
use crate::dmstring::DMString;

unsafe extern "thiscall" {
	#[cfg_attr(windows, link_name = "??0DMString@@QAE@XZ")]
	#[cfg_attr(target_os = "linux", link_name = "_ZN8DMStringC2Ev")]
	/// DMString::DMString();
	pub fn dm_string_new(this: *mut DMString) -> *mut DMString;
	#[cfg_attr(windows, link_name = "??1DMString@@QAE@XZ")]
	#[cfg_attr(target_os = "linux", link_name = "_ZN8DMStringD2Ev")]
	/// DMString::~DMString();
	pub fn dm_string_free(this: *mut DMString);
}

unsafe extern "stdcall" {
	#[cfg_attr(
		windows,
		link_name = "?GetServerMemUsage@DungServer@@QAE?AUDMString@@XZ"
	)]
	#[cfg_attr(
		target_os = "linux",
		link_name = "_ZN10DungServer17GetServerMemUsageEv"
	)]
	/// DungServer::GetServerMemUsage(DMString *out)
	pub fn get_server_mem_usage(out: *mut DMString);
}
