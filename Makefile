CARGO ?= cargo
CARGO_TARGET ?= x86_64-unknown-linux-musl
CARGO_BUILD_TYPE_RELEASE = --release

release-musl:
	$(CARGO) build $(CARGO_BUILD_TYPE_RELEASE) --target=$(CARGO_TARGET)
	$(CARGO) bloat $(CARGO_BUILD_TYPE_RELEASE) --target=$(CARGO_TARGET)
	$(CARGO) strip $(CARGO_BUILD_TYPE_RELEASE) --target=$(CARGO_TARGET)

.PHONY = release-musl
