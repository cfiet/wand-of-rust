all:
	rustc rustwand.rc

test:
	rustc --test rustwand.rc

clean:
	rm -rf rustwand *.o *.dylib *.dSYM
