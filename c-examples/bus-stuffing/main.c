#include "../vcsLib.h"

RAM_FUNC int elf_main(uint32_t* args)
{
	// Always reset PC first, cause it's going to be close to the end of the 6507 address space
	vcsJmp3();
	
	// Init TIA and RIOT RAM
	vcsLda2(0);
	for (int i = 0; i < 256; i++) {
		vcsSta3(i);
	}

    // Prepare registers for bus stuffing
	vcsLdaForBusStuff2();
	vcsLdxForBusStuff2();
	vcsLdyForBusStuff2();

    // Now we're able to do:
    //  vcsWrite3(ZP_ADDRESS, n);
    // instead of:
    //   vcsLda2(n);
	//	 vcsSta3(ZP_ADDRESS);

	while (1)
	{
		// 3 lines of VSYNC
		vcsWrite3(VSYNC, 2);
		for (int i = 0; i < 3; i++) {
			vcsSta3(WSYNC);
		}
		vcsWrite3(VSYNC, 0);

		// 37 lines of VBLANK
		for (int i = 0; i < 37; i++) {
			vcsSta3(WSYNC);
		}
		vcsWrite3(VBLANK, 0); // disable blanking

		// 192 lines of COLUBK
		for (int i = 0; i < 192; i++) {
            vcsWrite3(COLUBK, i);
            vcsJmp3();
            vcsSta3(WSYNC);
		}
	
		vcsWrite3(VBLANK, 2); // enter blanking

		// 30 lines of Overscan
		for (int i = 0; i < 30; i++) {
			vcsSta3(WSYNC);
		}
	}
}
