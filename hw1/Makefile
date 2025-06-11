.PHONY: clean
clean:
	@echo "Cleaning the project..."
	@cargo clean

.PHONY: lint
lint:
	@echo "Running linters..."
	@echo "Running clippy..."
	@cargo clippy
	@echo "Running rustfmt..."
	@cargo fmt -- --check

.PHONY: check
check:
	@echo "Checking the code..."
	@cargo check

.PHONY: fmt
fmt:
	@echo "Formatting the code..."
	@cargo fmt

.PHONY: build-debug
build-debug:
	@echo "Building in debug mode..."
	@cargo build

.PHONY: build-release
build-release:
	@echo "Building in release mode..."
	@cargo build --release

.PHONY: run
run:
	@echo "Running the application..."
	@cargo run




