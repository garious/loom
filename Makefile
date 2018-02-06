test:
	cargo test

bench:
	cargo bench

cov:
	docker run -it --rm --security-opt seccomp=unconfined --volume "$$(PWD):/volume" elmtai/docker-rust-kcov

fmt:
	cargo fmt -- --write-mode=diff

wfmt:
	cargo fmt -- --write-mode=overwrite
