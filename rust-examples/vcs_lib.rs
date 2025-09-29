//! Rust bindings for vcsLib - VCS/Atari 2600 library
//! 
//! This module provides Rust bindings for the VCS library functions and constants
//! used for Atari 2600 programming.

#![allow(dead_code)]

pub const MP_SYSTEM_TYPE: u8 = 0;
pub const MP_CLOCK_HZ: u8 = 1;
pub const MP_FEATURE_FLAGS: u8 = 2;
pub const MP_ELAPSED: u8 = 3; // Do not use, debug only, will be replaced in future
pub const MP_THRESHOLD: u8 = 4; // Do not use, debug only, will be replaced in future
pub const MP_COUNT: u8 = 5;

// MP_SYSTEM_TYPE values
pub const ST_NTSC_2600: u8 = 0;
pub const ST_PAL_2600: u8 = 1;
pub const ST_PAL60_2600: u8 = 2; // Note: PAL60 is used by the UCA carts to convey the user preference

// Feature flags
pub const FF_MULTI_CART: u8 = 1; // Indicates elf is loaded by multicart and should allow exiting (return from main() function)

// VCS/2600 memory mapped register constants
pub const VSYNC: u8 = 0x00;
pub const VBLANK: u8 = 0x01;
pub const WSYNC: u8 = 0x02;
pub const RSYNC: u8 = 0x03;
pub const NUSIZ0: u8 = 0x04;
pub const NUSIZ1: u8 = 0x05;
pub const COLUP0: u8 = 0x06;
pub const COLUP1: u8 = 0x07;
pub const COLUPF: u8 = 0x08;
pub const COLUBK: u8 = 0x09;
pub const CTRLPF: u8 = 0x0A;
pub const REFP0: u8 = 0x0B;
pub const REFP1: u8 = 0x0C;
pub const PF0: u8 = 0x0D;
pub const PF1: u8 = 0x0E;
pub const PF2: u8 = 0x0F;
pub const RESP0: u8 = 0x10;
pub const RESP1: u8 = 0x11;
pub const RESM0: u8 = 0x12;
pub const RESM1: u8 = 0x13;
pub const RESBL: u8 = 0x14;
pub const AUDC0: u8 = 0x15;
pub const AUDC1: u8 = 0x16;
pub const AUDF0: u8 = 0x17;
pub const AUDF1: u8 = 0x18;
pub const AUDV0: u8 = 0x19;
pub const AUDV1: u8 = 0x1A;
pub const GRP0: u8 = 0x1B;
pub const GRP1: u8 = 0x1C;
pub const ENAM0: u8 = 0x1D;
pub const ENAM1: u8 = 0x1E;
pub const ENABL: u8 = 0x1F;
pub const HMP0: u8 = 0x20;
pub const HMP1: u8 = 0x21;
pub const HMM0: u8 = 0x22;
pub const HMM1: u8 = 0x23;
pub const HMBL: u8 = 0x24;
pub const VDELP0: u8 = 0x25;
pub const VDELP1: u8 = 0x26;
pub const VDELBL: u8 = 0x27;
pub const RESMP0: u8 = 0x28;
pub const RESMP1: u8 = 0x29;
pub const HMOVE: u8 = 0x2A;
pub const HMCLR: u8 = 0x2B;
pub const CXCLR: u8 = 0x2C;

// Collision detection registers
pub const CXM0P: u8 = 0x00;
pub const CXM1P: u8 = 0x01;
pub const CXP0FB: u8 = 0x02;
pub const CXP1FB: u8 = 0x03;
pub const CXM0FB: u8 = 0x04;
pub const CXM1FB: u8 = 0x05;
pub const CXBLPF: u8 = 0x06;
pub const CXPPMM: u8 = 0x07;
pub const INPT0: u8 = 0x08;
pub const INPT1: u8 = 0x09;
pub const INPT2: u8 = 0x0A;
pub const INPT3: u8 = 0x0B;
pub const INPT4: u8 = 0x0C;
pub const INPT5: u8 = 0x0D;

// RIOT chip registers
pub const SWCHA: u16 = 0x0280;
pub const SWACNT: u16 = 0x0281;
pub const SWCHB: u16 = 0x0282;
pub const SWBCNT: u16 = 0x0283;
pub const INTIM: u16 = 0x0284;
pub const TIMINT: u16 = 0x0285;
pub const TIM1T: u16 = 0x0294;
pub const TIM8T: u16 = 0x0295;
pub const TIM64T: u16 = 0x0296;
pub const T1024T: u16 = 0x0297;

// External C function declarations from vcsLib
extern "C" {
    // Firmware use only
    pub static Ntsc2600: [u8; 256];
    pub static Pal2600: [u8; 256];
    pub static Ntsc7800: [u8; 256];
    pub static Pal7800: [u8; 256];
    pub fn vcsLibInit();
    pub fn vcsInitBusStuffing();

    // Firmware or game use
    pub static ColorLookup: [u8; 256];
    pub static ReverseByte: [u8; 256]; // Reverses the order of the bits. 7..0 becomes 0..7. Useful for PF0, PF2, and reflecting sprites in software.

    // Bus Stuffing - must load A, X, and Y prior to using Write3()
    pub fn vcsLdaForBusStuff2();
    pub fn vcsLdxForBusStuff2();
    pub fn vcsLdyForBusStuff2();
    pub fn vcsWrite3(zp: u8, data: u8);

    /// jmp $f000 - used to keep PC in range of ROM. Call this when there are spare cycles to kill
    pub fn vcsJmp3();

    // nop can be used to adjust timing of display kernel code, or to give ARM more time between servicing 6502 bus
    pub fn vcsNop2();
    pub fn vcsNop2n(n: u16);

    /// lda #, sta zp
    pub fn vcsWrite5(zero_page: u8, data: u8);
    /// lda #, sta abs
    pub fn vcsWrite6(address: u16, data: u8);

    pub fn vcsLda2(data: u8);
    pub fn vcsLdx2(data: u8);
    pub fn vcsLdy2(data: u8);

    /// uses undocumented sax opcode to store (A & X) to zero page
    pub fn vcsSax3(zero_page: u8);
    pub fn vcsSta3(zero_page: u8);
    pub fn vcsStx3(zero_page: u8);
    pub fn vcsSty3(zero_page: u8);

    pub fn vcsSta4(address: u16);
    pub fn vcsStx4(address: u16);
    pub fn vcsSty4(address: u16);

    pub fn vcsCopyOverblankToRiotRam();
    pub fn vcsStartOverblank();
    pub fn vcsEndOverblank();

    pub fn vcsRead4(address: u16) -> u8;
    pub fn randint() -> i32;

    // Stack operations for advanced kernels without the use of bus stuffing
    pub fn vcsTxs2();
    pub fn vcsJsr6(target: u16);
    pub fn vcsPha3();
    pub fn vcsPhp3();
    pub fn vcsPla4();
    pub fn vcsPlp4();
    // Can be used when SP is aimed at TIA registers to simultaneously load a register with a 6 bit value, and undo SP change of PHP PHA
    pub fn vcsPla4Ex(data: u8);
    pub fn vcsPlp4Ex(data: u8);

    // primarily used in 7800 games
    /// Transfer 6502 execution to 6502 RAM
    pub fn vcsJmpToRam3(addr: u16);
    /// spin lock arm until 6502 accesses the specified address
    pub fn vcsWaitForAddress(address: u16);
    pub fn injectDmaData(address: i32, count: i32, p_buffer: *const u8);
}

/// Helper function to generate a sleep of a specific number of cycles (must be > 1)
pub unsafe fn vcs_sleep(cycles: u16) {
    if cycles <= 1 {
        panic!("vcsSleep: cycles must be greater than 1");
    }
    
    if cycles == 3 {
        vcsJmp3();
    } else if cycles & 1 != 0 {
        // Odd number of cycles
        vcsJmp3();
        vcsNop2n((cycles - 3) >> 1);
    } else {
        // Even number of cycles
        vcsNop2n(cycles >> 1);
    }
}