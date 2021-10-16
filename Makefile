RUST_CONFIG=debug
ifeq ($(RUST_CONFIG), release)
	CONFIG_FLAG=--release
else
	CONFIG_FLAG=
endif

.PHONY: examples clean run

examples: hello

hello: examples/hello/main.rs examples/stack.obj dos.json
	cargo build $(CONFIG_FLAG) --example hello
	mkdir -p target/dos/$(RUST_CONFIG)

	PATH=$$WATCOM/binl:$$WATCOM/binw:$$PATH \
	wlink name target/dos/$(RUST_CONFIG)/hello \
	  system dos32x \
	  option dosseg \
	  option start=_start \
	  file examples/stack.obj \
	  library target/dos/$(RUST_CONFIG)/examples/libhello.a

examples/stack.obj: examples/stack.nasm
	nasm -f obj examples/stack.nasm -o examples/stack.obj

run: hello
	dosbox target/dos/debug/hello.exe
