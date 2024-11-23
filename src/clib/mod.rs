use crate::toml_parse::savefile::{Error as SfError, Savefile as SfSavefile};
use std::ffi::CStr;
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
}

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

type Savefile = *mut c_void;

#[no_mangle]
pub extern "C" fn read_savefile(path: *const c_char, savefile_out: *mut Savefile) -> c_int {
    if path.is_null() || savefile_out.is_null() {
        return Error::NullPtrError as c_int;
    }

    let rust_path = match unsafe { CStr::from_ptr(path) }.to_str() {
        Err(_) => return Error::FfiError as c_int,
        Ok(p) => p.to_string(),
    };

    let new_sf = match SfSavefile::read_from_file(rust_path) {
        Err(e) => return Error::to_c_int(e),
        Ok(s) => s,
    };

    unsafe {
        *savefile_out = Box::into_raw(Box::new(new_sf)).cast();
    }

    0
}
