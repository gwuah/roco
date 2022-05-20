release:
	@cargo build --release
	@rm -rf bin
	@mkdir bin
	@cp ./target/release/roco ./bin
	@echo "release complete"