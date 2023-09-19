SOURCE_FILES := $(shell test -e src/ && find src -type f)
VERSION := $(shell sed --posix -n 's,^version = \"\(.*\)\",\1,p' Cargo.toml)

policy.wasm: $(SOURCE_FILES) Cargo.*
	cargo build --target=wasm32-wasi --release
	cp target/wasm32-wasi/release/*.wasm policy.wasm

artifacthub-pkg.yml: metadata.yml Cargo.toml
	$(warning If you are updating the artifacthub-pkg.yml file for a release, \
	  remember to set the VERSION variable with the proper value. \
	  To use the latest tag, use the following command:  \
	  make VERSION=$$(git describe --tags --abbrev=0 | cut -c2-) annotated-policy.wasm)
	kwctl scaffold artifacthub --metadata-path metadata.yml --version $(VERSION) \
		--output artifacthub-pkg.yml

annotated-policy.wasm: policy.wasm metadata.yml
	kwctl annotate -m metadata.yml -u README.md -o annotated-policy.wasm policy.wasm

.PHONY: fmt
fmt:
	cargo fmt --all -- --check

.PHONY: lint
lint:
	cargo clippy -- -D warnings

.PHONY: e2e-tests
e2e-tests: annotated-policy.wasm
	bats e2e.bats

.PHONY: test
test: fmt lint
	cargo test

.PHONY: clean
clean:
	cargo clean
	rm -f policy.wasm annotated-policy.wasm artifacthub-pkg.yml
