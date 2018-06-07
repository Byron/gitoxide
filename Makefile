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
	$(info journey-tests                | run all stateless journey test)
	$(info continuous-journey-tests     | run all stateless journey test whenever something changes)

always:

interactive-developer-environment-in-docker:
	docker build -t $(docker_image) - < etc/developer.Dockerfile
	docker run -v $$PWD:/volume -w /volume -it $(docker_image)

target/debug/rit: always
	cargo build

target/release/rit: always
	cargo build --release

lint:
	cargo clippy

profile: target/release/rit
	valgrind --callgrind-out-file=callgrind.profile --tool=callgrind  $< $(bench_fixture) >/dev/null
	callgrind_annotate --auto=yes callgrind.profile

benchmark: target/release/rit
	hyperfine '$< $(bench_fixture)'

journey-tests: target/debug/rit
	./tests/stateless-journey.sh $<

continuous-journey-tests:
	watchexec $(MAKE) journey-tests

