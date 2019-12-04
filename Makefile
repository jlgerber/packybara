exename := pkb
setup := pkb-setupdb
build:
	cargo build --release

install:
	cp target/release/$(exename) ~/bin/.
	cp target/release/$(setup) ~/bin/.