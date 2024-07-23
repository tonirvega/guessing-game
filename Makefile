play:
	docker run -it --rm -v $(PWD):/usr/src/myapp -w /usr/src/myapp rust:latest cargo run

