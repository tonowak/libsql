#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

include!("../bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn from_cstr(mut cstr: *const ::std::os::raw::c_char) -> String {
        unsafe {
            let mut result = String::new();
            while *cstr != 0 {
                result.push((*cstr as u8) as char);
                cstr = cstr.offset(1);
            }
            result
        }
    }

    extern "C" fn callback(
        _data: *mut ::std::os::raw::c_void,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
        _azColName: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        unsafe {
            assert_eq!(argc, 1);
            assert_eq!(from_cstr(*argv), "SQL fuzz");
            0
        }
    }

    #[test]
    fn basicTest() {
        unsafe {
            let mut db: *mut sqlite3 = ::std::ptr::null_mut();
            let path = CString::new("../../test/fuzzdata1.db").unwrap();
            assert_eq!(sqlite3_open(path.as_ptr(), &mut db), 0);

            let sql = CString::new("SELECT msg FROM readme").unwrap();
            let rc: u32 = sqlite3_exec(
                db,
                sql.as_ptr(),
                Some(callback),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
            .try_into()
            .unwrap();

            if rc != SQLITE_OK {
                panic!("Error while executing SQL query.");
            }

            sqlite3_close(db);
        }
    }
}
