docker_image = docker_developer_environment

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

always:

##@ Docker Support

interactive-developer-environment-in-docker: ## Use docker for all dependencies - run make from there
	docker build -t $(docker_image) - < etc/developer.Dockerfile
	docker run -v $$PWD:/volume -w /volume -it $(docker_image)

##@ Development

target/release/gio: always
	cargo build --release --no-default-features --features lean-cli

lint: ## Run lints with clippy
	cargo clippy

profile: target/release/gio ## run callgrind and annotate its output - linux only
	valgrind --callgrind-out-file=callgrind.profile --tool=callgrind  $< >/dev/null
	callgrind_annotate --auto=yes callgrind.profile

benchmark: target/release/gio ## see how fast things are, powered by hyperfine
	hyperfine '$<'

tests: check unit-tests journey-tests journey-tests-lean-cli ## run all tests, including journey tests

check: ## Build all code in suitable configurations
	cargo check --all
	cargo check --all --all-features
	cargo check --no-default-features --features lean-cli
	cargo check --no-default-features --features pretty-cli
	cargo check --no-default-features --features lean-cli,fast
	cd git-features && cargo check --all-features \
			   && cargo check --features parallel \
			   && cargo check --features fast-sha1

unit-tests: ## run all unit tests
	cargo test --all --no-fail-fast

continuous-unit-tests: ## run all unit tests whenever something changes
	watchexec -w src $(MAKE) unit-tests

journey-tests: always  ## run stateless journey tests (pretty-cli)
	cargo build
	./tests/stateless-journey.sh target/debug/gio target/debug/gio-plumbing

journey-tests-lean-cli: always ## run stateless journey tests (lean-cli)
	cargo build --no-default-features --features lean-cli
	./tests/stateless-journey.sh target/debug/gio target/debug/gio-plumbing

continuous-journey-tests: ## run stateless journey tests whenever something changes
	watchexec $(MAKE) journey-tests

##@ Maintenance

update-assets: ## refresh assets compiles into the binaries from their source
	-rm -Rf git-core/assets/baseline-init
	mkdir -p git-core/assets
	cp -R tests/fixtures/baseline-init git-core/assets/baseline-init
