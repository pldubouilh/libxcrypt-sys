lint-enforce::
	cargo clippy --all -- -D warnings
	cargo fmt --all -- --check

build::
	mkdir -p libinstall
	cd libxcrypt &&\
		./autogen.sh &&\
		./configure --enable-static &&\
		make &&\
		make install DESTDIR=${PWD}/libinstall/
	cargo build

ci:: build lint-enforce
	echo ok

run::
	cargo run

publish:: lint
	cargo publish

lint::
	cargo clippy --all
	cargo fmt --all