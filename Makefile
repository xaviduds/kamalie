r:
	cargo run

procs:
	mprocs \
	"bacon . --job clippy-all" \
	"dx serve --hot-patch" \
	"mdbook serve"

release:
	cargo build --release
