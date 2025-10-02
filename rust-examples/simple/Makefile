main: main.rs
	rustc --target=thumbv6m-none-eabi \
		-C panic=abort \
		-C opt-level=3 \
		-C relocation-model=pic \
		-C link-arg=-r \
		-C link-arg=-u \
		-C link-arg=elf_main \
		-C link-arg=--discard-all \
		--emit=link \
		main.rs -o main.bin
	arm-none-eabi-strip main.bin -d -R.comment -R.ARM.attributes

clean:
	rm -f *.bin *.o