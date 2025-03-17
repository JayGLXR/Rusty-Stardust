NAME   := stardust

# Docker commands
DOCKER_BUILD := docker build -t stardust-rust .
DOCKER_RUN   := docker run --rm -v $(PWD):/stardust stardust-rust

# Cargo commands
CARGO := cargo

# Build directories
BIN_DIR := bin
OBJ_DIR := bin/obj

all: x64 x86
debug: x64-debug x86-debug

# Create necessary directories
dirs:
	@mkdir -p $(BIN_DIR) $(OBJ_DIR)

# Build Docker image
docker-build:
	$(DOCKER_BUILD)

# x64 builds
x64: dirs docker-build
	@echo "Compiling x64 project"
	$(DOCKER_RUN) $(CARGO) build --release --target x86_64-pc-windows-gnu
	@echo "Extracting shellcode"
	$(DOCKER_RUN) x86_64-w64-mingw32-objcopy --dump-section .text=$(BIN_DIR)/$(NAME).x64.bin target/x86_64-pc-windows-gnu/release/stardust.dll

x64-debug: dirs docker-build
	@echo "Compiling x64 project (debug)"
	$(DOCKER_RUN) $(CARGO) build --features debug --target x86_64-pc-windows-gnu
	@echo "Extracting shellcode"
	$(DOCKER_RUN) x86_64-w64-mingw32-objcopy --dump-section .text=$(BIN_DIR)/$(NAME).x64.bin target/x86_64-pc-windows-gnu/debug/stardust.dll

# x86 builds
x86: dirs docker-build
	@echo "Compiling x86 project"
	$(DOCKER_RUN) $(CARGO) build --release --target i686-pc-windows-gnu
	@echo "Extracting shellcode"
	$(DOCKER_RUN) i686-w64-mingw32-objcopy --dump-section .text=$(BIN_DIR)/$(NAME).x86.bin target/i686-pc-windows-gnu/release/stardust.dll

x86-debug: dirs docker-build
	@echo "Compiling x86 project (debug)"
	$(DOCKER_RUN) $(CARGO) build --features debug --target i686-pc-windows-gnu
	@echo "Extracting shellcode"
	$(DOCKER_RUN) i686-w64-mingw32-objcopy --dump-section .text=$(BIN_DIR)/$(NAME).x86.bin target/i686-pc-windows-gnu/debug/stardust.dll

clean:
	@rm -rf target
	@rm -rf $(BIN_DIR)/*.bin
	@echo "Cleaned build artifacts"

.PHONY: all debug x64 x86 x64-debug x86-debug clean dirs docker-build