#![feature(asm)]
use std::ffi::CString;

fn main() {
    let s = CString::new("Hello from syscall 1\n").unwrap();
    let l = s.as_bytes().len();

    unsafe {
        asm!(
            "syscall",
            in("rax") 1,  // syscall 1 is write
            in("rdi") 1,  // file handle 1 is stdout
            in("rdx") l,  // number of bytes in string
            in("rsi") s.as_ptr(),  // string address
        );
    }
}
