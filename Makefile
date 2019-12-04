exename := pkb
setup := pkb-setupdb

all: build install 

.PHONY: build
build:
	cargo build --release

.PHONY: install
install:
	cp target/release/$(exename) ~/bin/.
	cp target/release/$(setup) ~/bin/.

clean-targets:
	rm target/release/$(exename)
	rm target/release/$(setup)