
main: main.rs
	rustc --target=thumbv6m-none-eabi \
		-C panic=abort \
		-C opt-level=3 \
		-C relocation-model=pic \
		-C linker=arm-none-eabi-ld \
		-C link-arg=-r \
		-C link-arg=-u \
		-C link-arg=elf_main \
		-C link-arg=--strip-all \
		--emit=link \
		main.rs -o main.bin
	arm-none-eabi-strip main.bin -d -R.comment -R.ARM.attributes

# main: main.rs ../vcs_lib.rs
# 	rustc --target=thumbv6m-none-eabi \
# 		-C panic=abort \
# 		-C opt-level=3 \
# 		-C relocation-model=pic \
# 		--emit=obj \
# 		main.rs -o main.o
# 	arm-none-eabi-gcc -r main.o -o main.bin
# 	arm-none-eabi-strip main.bin -d -R.comment -R.ARM.attributes
# 	rm -f main.o

clean:
	rm -f *.bin *.o