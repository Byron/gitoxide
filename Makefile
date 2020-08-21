docker_image = docker_developer_environment

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

always:

##@ Docker Support

interactive-developer-environment-in-docker: ## Use docker for all dependencies - run make from there
	docker build -t $(docker_image) - < etc/developer.Dockerfile
	docker run -v $$PWD:/volume -w /volume -it $(docker_image)

##@ Release Builds

release-all: release release-lean release-small ## all release builds

release: always ## the default build, big but pretty (builds in ~2min 35s)
	cargo build --release

release-unix: always ## the default build, big but pretty, unix only (builds in ~2min 35s)
	cargo build --release --no-default-features --features max-termion

release-lean: always ## lean and fast, with line renderer (builds in ~1min 30s)
	cargo build --release --no-default-features --features lean

release-light: always ## lean and fast, log only (builds in ~1min 14s)
	cargo build --release --no-default-features --features light

release-small: always ## minimal dependencies, at cost of performance (builds in ~46s)
	cargo build --release --no-default-features --features small

##@ Debug Builds

debug: always ## the default build, big but pretty
	cargo build

debug-unix: always ## the default build, big but pretty, unix only
	cargo build --no-default-features --features max-termion

debug-lean: always ## lean and fast, with line renderer
	cargo build --no-default-features --features lean

debug-light: always ## lean and fast
	cargo build --no-default-features --features light

debug-small: always ## minimal dependencies, at cost of performance
	cargo build --no-default-features --features small

##@ Development

target/release/gix: always
	cargo build --release --no-default-features --features small

lint: ## Run lints with clippy
	cargo clippy

profile: target/release/gix ## run callgrind and annotate its output - linux only
	valgrind --callgrind-out-file=callgrind.profile --tool=callgrind  $< >/dev/null
	callgrind_annotate --auto=yes callgrind.profile

benchmark: target/release/gix ## see how fast things are, powered by hyperfine
	hyperfine '$<'

##@ Testing

tests: clippy check unit-tests journey-tests-small journey-tests ## run all tests, including journey tests

clippy: ## Run cargo clippy on all crates
	cargo clippy --all

check: ## Build all code in suitable configurations
	cargo check --all
	cargo check --all --all-features
	cargo check --no-default-features --features small
	cargo check --no-default-features --features light
	cargo check --no-default-features --features lean
	cargo check --no-default-features --features lean-termion
	cargo check --no-default-features --features max
	cargo check --no-default-features --features max-termion
	cd gitoxide-core && cargo check --all-features \
                     && cargo check
	cd git-object && cargo check --all-features
	cd git-odb && cargo check --all-features \
			   && cargo check
	cd git-packetline && cargo check --all-features \
			   && cargo check
	cd git-protocol && cargo check --all-features \
			   && cargo check
	cd git-url && cargo check --all-features \
			   && cargo check
	cd git-features && cargo check --all-features \
			   && cargo check --features parallel \
			   && cargo check --features fast-sha1 \
			   && cargo check --features interrupt-handler \
			   && cargo check --features disable-interrupts

unit-tests: ## run all unit tests
	cargo test --all --no-fail-fast
	cd git-features && cargo test && cargo test --features fast-sha1

continuous-unit-tests: ## run all unit tests whenever something changes
	watchexec -w src $(MAKE) unit-tests

jtt = tests/tools/target/debug/jtt
journey-tests: always  ## run stateless journey tests (max)
	cargo build
	cd tests/tools && cargo build
	./tests/stateless-journey.sh target/debug/gix target/debug/gixp $(jtt) max

journey-tests-small: always ## run stateless journey tests (lean-cli)
	cargo build --no-default-features --features small
	cd tests/tools && cargo build
	./tests/stateless-journey.sh target/debug/gix target/debug/gixp $(jtt) small

continuous-journey-tests: ## run stateless journey tests whenever something changes
	watchexec $(MAKE) journey-tests

rust_repo = tests/fixtures/repos/rust.git
$(rust_repo):
	mkdir -p $@
	cd $@ && git init --bare && git remote add origin https://github.com/rust-lang/rust && git fetch

linux_repo = tests/fixtures/repos/linux.git
$(linux_repo):
	mkdir -p $@
	cd $@ && git init --bare && git remote add origin https://github.com/torvalds/linux && git fetch

##@ on CI

stress: ## Run various algorithms on big repositories
	$(MAKE) -j3 $(linux_repo) $(rust_repo) release-lean
	time ./target/release/gixp --verbose pack-verify --re-encode $(linux_repo)/objects/pack/*.idx
	mkdir out && time ./target/release/gixp --verbose pack-index-from-data -p $(linux_repo)/objects/pack/*.pack out/
	time ./target/release/gixp --verbose pack-verify out/*.idx

	time ./target/release/gixp --verbose pack-verify --statistics $(rust_repo)/objects/pack/*.idx
	time ./target/release/gixp --verbose pack-verify --algorithm less-memory $(rust_repo)/objects/pack/*.idx
	time ./target/release/gixp --verbose pack-verify --re-encode $(rust_repo)/objects/pack/*.idx
	time ./target/release/gixp --verbose pack-explode .git/objects/pack/*.idx

	rm -Rf delme; mkdir delme && time ./target/release/gixp --verbose pack-explode .git/objects/pack/*.idx delme/

##@ Maintenance

baseline_asset_dir = git-repository/src/assets/baseline-init
baseline_asset_fixture = tests/fixtures/baseline-init

$(baseline_asset_fixture):
	mkdir -p $@
	cd "$@" && git init --bare && \
		sed -i '' -E '/bare = true|ignorecase = true|precomposeunicode = true|filemode = true/d' config && \
		sed -i '' 's/master/main/g' $$(find . -type f)

transport_fixtures = git-transport/tests/fixtures
base_url = https://github.com/Byron/gitoxide.git
update-curl-fixtures: ## use curl to fetch raw fixtures for use in unit test. Changes there might break them
	curl -D - -L "$(base_url)/info/refs?service=git-upload-pack"  > $(transport_fixtures)/v1/http-handshake.response
	curl -D - -H 'Git-Protocol: version=2' -L "$(base_url)/info/refs?service=git-upload-pack"  > $(transport_fixtures)/v2/http-handshake.response

update-assets: $(baseline_asset_fixture) ## refresh assets compiled into the binaries from their source
	-rm -Rf $(baseline_asset_dir)
	mkdir -p $(dir $(baseline_asset_dir))
	cp -R $< $(baseline_asset_dir)

force-update-assets: ## As update-assets, but will run git to update the baseline as well
	-rm -Rf $(baseline_asset_fixture)
	$(MAKE) update-assets

check-size: ## Run cargo-diet on all crates to see that they are still in bound
	./etc/check-package-size.sh
