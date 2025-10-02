main: main.c
	arm-none-eabi-gcc -r -mlong-calls -fno-exceptions -march=armv6-m main.c -o main.bin -O3 -Wall -lgcc
	arm-none-eabi-strip main.bin -d -R.comment -R.ARM.attributes

clean:
	rm -f *.bin *.o