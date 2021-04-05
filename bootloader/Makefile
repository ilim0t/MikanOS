all: hello.efi

%.o: %.c
	clang -target x86_64-pc-win32-coff -o $@ -c $<

%.efi: %.o
	lld-link /subsystem:efi_application /entry:EfiMain /out:$@ $<

clean:
	rm -f hello.efi