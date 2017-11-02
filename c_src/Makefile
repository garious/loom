sha256_avx1:*.asm *.c
	gcc -O4 -o $@ main.c $@.o

sha256_sse4:*.asm *.c
	gcc -O4 -Dsha256=sha256_sse4 -o $@ main.c $@.o

%.o:%.asm
	yasm -f x64 -f elf64 -X gnu -g dwarf2 -D LINUX -o $@ $^
