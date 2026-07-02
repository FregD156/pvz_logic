# Makefile to quickly build and run the Plants vs. Zombies Bevy logic prototype.

.PHONY: run build check clean help

# Run the game
run:
	@. $(HOME)/.cargo/env && cargo run --manifest-path=pvz_logic/Cargo.toml --bin pvz_logic

# Build the game binary
build:
	@. $(HOME)/.cargo/env && cargo build --manifest-path=pvz_logic/Cargo.toml

# Check compilation errors
check:
	@. $(HOME)/.cargo/env && cargo check --manifest-path=pvz_logic/Cargo.toml

# Clean compilation target files
clean:
	@. $(HOME)/.cargo/env && cargo clean --manifest-path=pvz_logic/Cargo.toml

# Run the peashooter animation demo (empty scene)
demo:
	@. $(HOME)/.cargo/env && cargo run --manifest-path=pvz_logic/Cargo.toml --bin peashooter_demo

# Run the grass sod rolling out demo
sod:
	@. $(HOME)/.cargo/env && cargo run --manifest-path=pvz_logic/Cargo.toml --bin sod_roll_demo

# Show help options
help:
	@echo "Available commands:"
	@echo "  make run   - Compile and launch the Bevy prototype"
	@echo "  make demo  - Launch the Peashooter animation debugger (empty scene)"
	@echo "  make build - Build target executable"
	@echo "  make check - Run cargo compiler check"
	@echo "  make clean - Remove compiled build targets"
