r:
	cargo run

procs:
	mprocs \
	"bacon . --job clippy-all" \
	"cargo run"

release:
	cargo build --release
