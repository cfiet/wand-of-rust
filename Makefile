all: lib samples

lib: librustwand-c6deddd8be3da891-0.1.dylib

librustwand-c6deddd8be3da891-0.1.dylib:
	rustc rustwand.rc

test:
	rustc --test rustwand.rc

samples: lib
	rustc -L . samples/thumbnail.rs -o thumbnail.sample

clean:
	rm -rf rustwand *.o *.dylib *.dSYM  samples/*.sample samples/*.dSYM
