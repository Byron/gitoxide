docker_image = docker_developer_environment

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

always:

##@ Docker Support

interactive-developer-environment-in-docker: ## Use docker for all dependencies - run make from there
	docker build -t $(docker_image) - < etc/developer.Dockerfile
	docker run -v $$PWD:/volume -w /volume -it $(docker_image)

##@ Release Builds

release-default: always ## the default build, big but pretty (builds in ~2min 35s)
	cargo build --release

release-lean: always ## lean and fast (builds in ~1min 10s)
	cargo build --release --no-default-features --features lean

release-small: always ## minimal dependencies, at cost of performance (builds in ~46s)
	cargo build --release --no-default-features --features small

##@ Debug Builds

debug-default: always ## the default build, big but pretty
	cargo build

debug-lean: always ## lean and fast
	cargo build --release --no-default-features --features lean

debug-small: always ## minimal dependencies, at cost of performance
	cargo build --release --no-default-features --features small

##@ Development

target/release/gio: always
	cargo build --release --no-default-features --features small

lint: ## Run lints with clippy
	cargo clippy

profile: target/release/gio ## run callgrind and annotate its output - linux only
	valgrind --callgrind-out-file=callgrind.profile --tool=callgrind  $< >/dev/null
	callgrind_annotate --auto=yes callgrind.profile

benchmark: target/release/gio ## see how fast things are, powered by hyperfine
	hyperfine '$<'

##@ Testing

tests: check unit-tests journey-tests-small journey-tests ## run all tests, including journey tests

check: ## Build all code in suitable configurations
	cargo check --all
	cargo check --all --all-features
	cargo check --no-default-features --features small
	cargo check --no-default-features --features light
	cargo check --no-default-features --features lean
	cargo check --no-default-features --features lean-termion
	cargo check --no-default-features --features max
	cargo check --no-default-features --features max-termion
	cd gitoxide-core && cargo check --all-features
	cd git-object && cargo check --all-features
	cd git-odb && cargo check --all-features
	cd git-features && cargo check --all-features \
			   && cargo check --features parallel \
			   && cargo check --features fast-sha1

unit-tests: ## run all unit tests
	cargo test --all --no-fail-fast

continuous-unit-tests: ## run all unit tests whenever something changes
	watchexec -w src $(MAKE) unit-tests

journey-tests: always  ## run stateless journey tests (max)
	cargo build
	./tests/stateless-journey.sh target/debug/gio target/debug/giop max

journey-tests-small: always ## run stateless journey tests (lean-cli)
	cargo build --no-default-features --features small
	./tests/stateless-journey.sh target/debug/gio target/debug/giop small

continuous-journey-tests: ## run stateless journey tests whenever something changes
	watchexec $(MAKE) journey-tests

rust_repo = tests/fixtures/repos/rust
$(rust_repo):
	mkdir -p $@
	cd $@ && git init && git remote add origin https://github.com/rust-lang/rust && git fetch

stress: ## Run various algorithms on big repositories
	$(MAKE) -j2 $(rust_repo) release-lean
	time ./target/release/giop verify-pack --verbose $(rust_repo)/.git/objects/pack/*.idx
	time ./target/release/giop verify-pack --verbose --statistics $(rust_repo)/.git/objects/pack/*.idx

##@ Maintenance

baseline_asset_dir = git-repository/src/assets/baseline-init
baseline_asset_fixture = tests/fixtures/baseline-init

$(baseline_asset_fixture):
	mkdir -p $@
	cd "$@" && git init --bare && \
		sed -i '' -E '/bare = true|ignorecase = true|precomposeunicode = true|filemode = true/d' config && \
		sed -i '' 's/master/main/g' $$(find . -type f)


update-assets: $(baseline_asset_fixture) ## refresh assets compiles into the binaries from their source
	-rm -Rf $(baseline_asset_dir)
	mkdir -p $(dir $(baseline_asset_dir))
	cp -R $< $(baseline_asset_dir)

force-update-assets: ## As update-assets, but will run git to update the baseline as well
	-rm -Rf $(baseline_asset_fixture)
	$(MAKE) update-assets

check-size: ## Run cargo-diet on all crates to see that they are still in bound
	./etc/check-package-size.sh
