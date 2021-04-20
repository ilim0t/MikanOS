all: kernel.elf

%.o: %.cpp
#	clang++ -O2 -Wall -g --target=x86_64-elf -ffreestanding -mno-red-zone -fno-exceptions -fno-rtti -std=c++17 -c $<
	clang++ -O2 ${CPPFLAGS} --target=x86_64-elf -fno-exceptions -ffreestanding -c $<

%.elf: %.o
	ld.lld ${LDFLAGS} --entry KernelMain -z norelro --image-base 0x100000 --static -o $@ $<

clean:
	rm -f kernel.elf