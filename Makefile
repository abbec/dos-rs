CARGO_FLAGS=
RUSTFLAGS=
RUST_CONFIG=debug
ifeq ($(RUST_CONFIG), release)
	CONFIG_FLAG=--release
else
	CONFIG_FLAG=
endif

.PHONY: examples clean run

examples: hello

hello: examples/hello/main.rs stack.o dos.json
	export RUSTFLAGS="$(RUSTFLAGS)"
	cargo $(CARGO_FLAGS) build $(CONFIG_FLAG) --example hello
	mkdir -p target/dos/$(RUST_CONFIG)

	PATH=$$WATCOM/binl:$$WATCOM/binw:$$PATH \
	wlink name target/dos/$(RUST_CONFIG)/hello \
	  system dos32x \
	  option verbose \
	  option dosseg \
	  option start=_start \
	  file stack.obj \
	  library target/dos/$(RUST_CONFIG)/examples/libhello.a

stack.o: stack.nasm
	nasm -f obj stack.nasm

run: hello
	dosbox target/dos/debug/hello.exe
