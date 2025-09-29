#![no_std]
#![no_main]
#![no_builtins]

#[path = "../vcs_lib.rs"]
mod vcs_lib;
use vcs_lib::*;
use core::panic::PanicInfo;

// Define a panic handler (required with no_std)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn elf_main(_args: *mut u32) -> i32 {
    unsafe {
        // Always reset PC first, cause it's going to be close to the end of the 6507 address space
        vcsJmp3();
        
        // Init TIA and RIOT RAM
        vcsLda2(0);
        for i in 0..256 {
            vcsSta3(i as u8);
        }

        loop {
            // 3 lines of VSYNC
            vcsLda2(2);
            vcsSta3(VSYNC);
            for _ in 0..3 {
                vcsSta3(WSYNC);
            }
            vcsLda2(0);
            vcsSta3(VSYNC);

            // 37 lines of VBLANK
            for _ in 0..37 {
                vcsSta3(WSYNC);
            }
            vcsSta3(VBLANK); // disable blanking

            // 192 lines of COLUBK
            for i in 0..192 {
                vcsLdx2(i as u8);
                vcsStx3(COLUBK);
                vcsJmp3();
                vcsSta3(WSYNC);
            }
        
            vcsWrite5(VBLANK, 2); // enter blanking

            // 30 lines of Overscan
            for _ in 0..30 {
                vcsSta3(WSYNC);
            }
        }
    }
}