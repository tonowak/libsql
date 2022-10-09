#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

include!("../bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prosty_test() {
        unsafe {
            println!("{}", *sqlite3_libversion());
            assert!(SQLITE_IGNORE == 2);
        }
    }
}
