fixture = tests/fixtures/input.txt
bench_fixture = tests/fixtures/big-input.txt
docker_image = gitrs_docker_developer_environment

help:
	$(info -Targets -----------------------------------------------------------------------------)
	$(info -- Use docker for all dependencies - run make interactively from there ----------------)
	$(info interactive-developer-environment-in-docker | gives you everything you need to run all targets)
	$(info -Development Targets -----------------------------------------------------------------)
	$(info lint                         | run lints with clippy)
	$(info benchmark                    | just for fun, really)
	$(info profile                      | only on linux - run callgrind and annotate it)
	$(info unit-tests                   | run all unit tests)
	$(info journey-tests                | run all stateless journey test)
	$(info continuous-unit-tests        | run all unit tests whenever something changes)
	$(info continuous-journey-tests     | run all stateless journey test whenever something changes)

always:

interactive-developer-environment-in-docker:
	docker build -t $(docker_image) - < etc/developer.Dockerfile
	docker run -v $$PWD:/volume -w /volume -it $(docker_image)

target/debug/grit: always
	cargo build

target/release/grit: always
	cargo build --release

lint:
	cargo clippy

profile: target/release/grit
	valgrind --callgrind-out-file=callgrind.profile --tool=callgrind  $< $(bench_fixture) >/dev/null
	callgrind_annotate --auto=yes callgrind.profile

benchmark: target/release/grit
	hyperfine '$< $(bench_fixture)'

journey-tests: target/debug/grit
	./tests/stateless-journey.sh $<

unit-tests: 
	cd lib/git-odb && cargo test --tests

continuous-journey-tests:
	watchexec $(MAKE) journey-tests

continuous-unit-tests:
	watchexec -i '*target*' $(MAKE) unit-tests
