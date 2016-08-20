extern crate libc;

use libc::{c_void, c_char, c_int};

pub type Hunhandle = *mut c_void;

extern {
	pub fn Hunspell_create(affpath: *const c_char, dpath: *const c_char) -> Hunhandle;
	pub fn Hunspell_create_key(affpath: *const c_char, dpath: *const c_char, key: *const c_char) -> Hunhandle;
	pub fn Hunspell_destroy(pHunspell: Hunhandle) -> ();
	pub fn Hunspell_add_dic(pHunspell: Hunhandle, dpath: *const c_char) -> c_int;
	pub fn Hunspell_spell(pHunspell: Hunhandle, word: *const c_char) -> c_int;
	pub fn Hunspell_get_dic_encoding(pHunspell: Hunhandle) -> *const c_char;
	pub fn Hunspell_suggest(pHunspell: Hunhandle, slst: *mut *mut *mut c_char, word: *const c_char) -> c_int;
	pub fn Hunspell_analyze(pHunspell: Hunhandle, slst: *mut *mut *mut c_char, word: *const c_char) -> c_int;
	pub fn Hunspell_stem(pHunspell: Hunhandle, slst: *mut *mut *mut c_char, word: *const c_char) -> c_int;
	pub fn Hunspell_stem2(pHunspell: Hunhandle,
						  slst: *mut *mut *mut c_char,
						  desc: *mut *mut c_char,
						  n: c_int) -> c_int;
	pub fn Hunspell_generate(pHunspell: Hunhandle,
							 slst: *mut *mut *mut c_char,
							 word: *const c_char,
							 word2: *const c_char) -> c_int;
	pub fn Hunspell_generate2(pHunspell: Hunhandle,
							  slst: *mut *mut *mut c_char,
							  word: *const c_char,
							  desc: *mut *mut c_char,
							  n: c_int) -> c_int;
	pub fn Hunspell_add(pHunspell: Hunhandle, word: *const c_char) -> c_int;
	pub fn Hunspell_add_with_affix(pHunspell: Hunhandle, word: *const c_char, example: *const c_char) -> c_int;
	pub fn Hunspell_remove(pHunspell: Hunhandle, word: *const c_char) -> c_int;
	pub fn Hunspell_free_list(pHunspell: Hunhandle, slst: *mut *mut *mut c_char, n: c_int) -> ();
}
