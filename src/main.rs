extern crate libc;
use libc::c_int;
extern "system" {
    fn execute_hello_kernel(value: c_int);
}

fn main() {
    unsafe { execute_hello_kernel(1234); }
}
