use crate::randomizer::characters::Character;
use crate::randomizer::pool::Unlocks;
use crate::randomizer::targets::Target;
use crate::toml_parse::savefile::{Error as SfError, Savefile as SfSavefile};
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_int, c_void};

#[repr(C)]
pub enum Error {
    NoError = 0,
    IoError,
    SerializationError,
    DeserializationError,
    ParseError,
    FfiError,
    NullPtrError,
    CouldNotRollError,
}

macro_rules! check_null(
    () => (None);
    ( $x: expr ) => {
        if $x.is_null() {
            return Error::NullPtrError as c_int;
        }
    };
    ( $x: expr, $($xs: expr), * ) => {
        if $x.is_null() $(|| $xs.is_null())* {
            return Error::NullPtrError as c_int;
        }
    };
);

impl Error {
    fn to_c_int(sf_err: SfError) -> c_int {
        match sf_err {
            SfError::SerializeError(_) => return Error::SerializationError as c_int,
            SfError::DeserializeError(_) => return Error::DeserializationError as c_int,
            SfError::ParseError => return Error::ParseError as c_int,
            SfError::IoError(_) => return Error::IoError as c_int,
        }
    }
}

type UnlocksHandle = *mut c_void;
type RustString = *mut c_char;

#[no_mangle]
pub extern "C" fn read_unlocks_from_file(
    path: *const c_char,
    unlocks_out: *mut UnlocksHandle,
) -> c_int {
    check_null!(path, unlocks_out);

    let rust_path = match unsafe { CStr::from_ptr(path) }.to_str() {
        Err(_) => return Error::FfiError as c_int,
        Ok(p) => p.to_string(),
    };

    let new_sf = match SfSavefile::read_from_file(rust_path) {
        Err(e) => return Error::to_c_int(e),
        Ok(s) => s,
    };

    let new_unlocks: Unlocks = match new_sf.try_into() {
        Err(_) => return Error::ParseError as c_int,
        Ok(u) => u,
    };

    unsafe {
        *unlocks_out = Box::into_raw(Box::new(new_unlocks)).cast();
    }

    0
}

#[no_mangle]
pub extern "C" fn free_unlocks(unlocks_handle: UnlocksHandle) -> c_int {
    check_null!(unlocks_handle);

    unsafe {
        drop(Box::from_raw(unlocks_handle.cast::<Unlocks>()));
    }

    0
}

#[repr(C)]
pub struct RunTarget {
    character: Character,
    targets: c_int,
}

#[no_mangle]
pub extern "C" fn randomize(unlocks_handle: UnlocksHandle, targets_out: *mut RunTarget) -> c_int {
    check_null!(unlocks_handle, targets_out);

    let unlocks = unsafe { Box::<Unlocks>::from_raw(unlocks_handle.cast()) };

    let pick = match unlocks.get_random_pick() {
        Some(p) => p,
        None => return Error::CouldNotRollError as c_int,
    };

    let targets = unsafe { &mut *targets_out };
    targets.character = pick.0;
    targets.targets = 0;

    for target in pick.1 {
        targets.targets |= 1 << (target as u32);
    }

    Box::leak(unlocks);
    0
}

fn print_displayable(ds: &impl fmt::Display) -> Result<RustString, Error> {
    let c_char = match CString::new(ds.to_string()) {
        Err(_) => return Err(Error::FfiError),
        Ok(c) => c,
    };

    Ok(c_char.into_raw())
}

#[no_mangle]
pub extern "C" fn print_character(char: Character, str: *mut RustString) -> c_int {
    check_null!(str);

    unsafe {
        *str = match print_displayable(&char) {
            Err(e) => return e as c_int,
            Ok(d) => d,
        };
    }

    0
}

#[no_mangle]
pub extern "C" fn print_target(targ: Target, str: *mut RustString) -> c_int {
    check_null!(str);

    unsafe {
        *str = match print_displayable(&targ) {
            Err(e) => return e as c_int,
            Ok(d) => d,
        };
    }

    0
}

#[no_mangle]
pub extern "C" fn free_string(str: RustString) -> c_int {
    check_null!(str);

    unsafe { drop(CString::from_raw(str)) };

    0
}
