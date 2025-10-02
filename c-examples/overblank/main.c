#include "../vcsLib.h"

#define OVERSCAN_LINES 30
#define VSYNC_LINES 3 // 3 scanlines VSYNC
#define VBLANK_LINES 37

RAM_FUNC int elf_main(uint32_t* args)
{
	// Always reset PC first, cause it's going to be close to the end of the 6507 address space
	vcsJmp3();
	
	// Init TIA and RIOT RAM
	vcsLda2(0);
	for (int i = 0; i < 256; i++) {
		vcsSta3(i);
	}

	vcsCopyOverblankToRiotRam();
    vcsWrite5(0x96, OVERSCAN_LINES);
    vcsWrite5(0x9e, OVERSCAN_LINES + VSYNC_LINES);
    vcsWrite5(0xa6, OVERSCAN_LINES + VSYNC_LINES + VBLANK_LINES);
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
        vcsStartOverblank();

        //
		// run overblank game logic here (read joystick, etc.)
		//
	}
}
