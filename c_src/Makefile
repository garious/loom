sha256_avx1:*.asm *.c sha256_avx1.o
	gcc -O4 -o $@ main.c $@.o

sha256_sse4:*.asm *.c sha256_sse4.o
	gcc -O4 -Dsha256=sha256_sse4 -o $@ main.c $@.o

sha256_avx2_rorx8:*.asm *.c sha256_avx2_rorx8.o
	gcc -O4 -Dsha256=sha256_rorx_x8ms -o $@ main.c $@.o

sha256_avx2_rorx2:*.asm *.c sha256_avx2_rorx2.o
	gcc -O4 -Dsha256=sha256_rorx -o $@ main.c $@.o

%.o:%.asm
	yasm -f x64 -f elf64 -X gnu -g dwarf2 -D LINUX -o $@ $^
