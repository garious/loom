test:
	cargo test

integration:release help_t test_acc_t


release:
	cargo build --all-targets --release

bench:
	cargo +nightly bench --features="unstable"

cov:
	docker run -it --rm --security-opt seccomp=unconfined --volume "$$PWD:/volume" elmtai/docker-rust-kcov

fmt:
	cargo fmt -- --write-mode=diff

wfmt:
	cargo fmt -- --write-mode=overwrite

help_t:
	./target/release/loomd -h
	./target/release/loom -h


test_acc_t:
	./target/release/loomd -t ./testdata/test_accounts.json
