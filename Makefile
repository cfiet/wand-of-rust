all: librustwand samples

librustwand:
	rustc --lib src/rustwand/lib.rs -o lib/foo.so

samples:
	rustc -L ./lib/ src/samples/thumbnail/main.rs -o bin/thumbnail

clean: 
	rm -Rf ./lib/*.so ./bin/*
