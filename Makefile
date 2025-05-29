r:
	cargo run

procs:
	mprocs "bacon . --job clippy-all" "bacon . --job dev"

release:
	cargo build --release
