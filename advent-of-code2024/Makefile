day%: day%.asm
	nasm -f elf64 -Fdwarf -o $@.o $^
	ld -o $@ $@.o
