#![feature(asm)]

const MAP_PRIVATE: u64 = 0x02;
const MAP_ANON: u64 = 0x20;
const PROT_READ: u64 = 0x1;
const PROT_WRITE: u64 = 0x2;


// https://linuxhint.com/using_mmap_function_linux/
fn main() {
    let res: i64;

    unsafe {
        asm!(
            "syscall",
            in("rax") 9,  // syscall 9 is mmap
            in("rdi") 0,  // any address
            in("rsi") 4096,  // size of memory block
            in("rdx") PROT_READ | PROT_WRITE,  // protection flags
            in("r10") MAP_PRIVATE | MAP_ANON,  // mmap flags
            in("r8") 0,  // file handler, in case MAP_ANON should be 0
            in("r9") 0,  // offset in file, in case MAP_ANON should be 0
            lateout("rax") res,
        );
    }

    println!("mmap allocated 4096 mem block at {:#x?}", res);
}
