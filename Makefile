exename := pkb

build:
	cargo build --release

install:
	cp target/release/$(exename) ~/bin/.