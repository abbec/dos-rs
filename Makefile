CARGO_FLAGS=-Z build-std=core

hello: examples/hello/main.rs stack.o dos.json
	cargo $(CARGO_FLAGS) build --example hello
	mkdir -p target/dos/debug

	PATH=$$WATCOM/binl:$$WATCOM/binw:$$PATH \
	wlink name target/dos/debug/hello \
	  system dos32x \
	  option verbose \
	  option map=target/dos/debug/hello.map \
	  option dosseg \
	  option start=_start \
	  file stack.obj \
	  library target/dos/debug/examples/libhello.a

stack.o: stack.nasm
	nasm -f obj stack.nasm
