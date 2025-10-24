#include "../vcsLib.h"

// note: sizeof(CustomOverblank) MUST be exactly 49 bytes and end on "jmp $1000"
static const uint8_t CustomOverblank[] =
{
    0xea,				//   nop            ; add nops to make sure the code is 49 bytes long
	0xea,				//   nop
	0xea,				//   nop
	0xea,				//   nop
	0xea,				//   nop
	0xea,				//   nop
	0xea,				//   nop
	0xea,				//   nop
	0xea,				//   nop
	0xea,				//   nop
	0xea,				//   nop
	0xea,				//   nop
	0xea,				//   nop
	0xa0, 0x25,			//   ldy #$25        ; 37 lines over Overscan
	0xa2, 0x02,			//   ldx #2
						// Overscan:   
	0x85, 0x02,			//   sta WSYNC       ; wait for next scanline
	0x88,				//   dey
	0xd0, 0xfb, 		//   bne Overscan
						// VerticalSync:
	0x86, 0x00,			//   stx VSYNC
	0x85, 0x02,			//   sta WSYNC
	0x85, 0x02,			//   sta WSYNC
	0x85, 0x02,			//   sta WSYNC
	0xa2, 0x00,			//   ldx #0
	0x86, 0x00,			//   stx VSYNC
	0xa0, 0x1e,			//   ldy #$1e        ; 30 lines of Vertical Blank
						// VerticalBlank:   
	0x85, 0x02,			//   sta WSYNC       ; wait for next scanline
	0x88,				//   dey
	0xd0, 0xfb,			//   bne VerticalBlank
						// WaitForCart:
	0xae, 0xff, 0x1f,	//   ldx $1fff
	0xd0, 0xfb,			//   bne WaitForCart
	0x4c, 0x00, 0x10	//   jmp $1000
};

RAM_FUNC void vcsCopyCustomOverblankToRiotRam()
{
	for(int i = 0; i < sizeof(CustomOverblank); i++)
	{
		vcsWrite5((uint8_t)(0x80 + i), CustomOverblank[i]);
	}
}

RAM_FUNC int elf_main(uint32_t* args)
{
	// Always reset PC first, cause it's going to be close to the end of the 6507 address space
	vcsJmp3();
	
	// Init TIA and RIOT RAM
	vcsLda2(0);
	for (int i = 0; i < 256; i++) {
		vcsSta3(i);
	}

	vcsCopyCustomOverblankToRiotRam();
    vcsStartOverblank();

    //
    // initialize your game here
    //

	while (1)
	{
		vcsEndOverblank();

		vcsWrite5(VBLANK, 0);

		// 192 lines of COLUBK
		for (int i = 0; i < 192; i++) {
            vcsLdx2(i);
            vcsStx3(COLUBK);
            vcsJmp3();
            vcsSta3(WSYNC);
		}
		vcsLdx2(0);
        vcsStx3(COLUBK);

		vcsWrite5(VBLANK, 2); // enter blanking

		//read controller and switch values from RIOT
        // uint8_t INPT4_val = vcsRead4(INPT4);
        // uint8_t SWCHA_val = vcsRead4(SWCHA);
        // uint8_t SWCHB_val = vcsRead4(SWCHB);

        vcsStartOverblank();

        //
		// run overblank game logic here (handle joystick input, etc.)
		//
	}
}
