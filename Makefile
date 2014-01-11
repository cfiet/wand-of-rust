all: librustwand samples

librustwand:
	rustc --lib src/rustwand/lib.rs -o lib/foo.so

samples:
	# ImageMagick requires that you use the MagickWand-config utility
	# to determine the specific libraries to link. We need to pass this
	# as a single argument to --link-args for rustc, hence the quotes 
	# surrounding the backticks
	rustc -L ./lib/ src/samples/thumbnail/main.rs -o bin/thumbnail \
		--link-args "`MagickWand-config  --libs)`" 

clean: 
	rm -Rf ./lib/*.so ./bin/*
