all: build run

build:
	@mkdir -p public && cp -r ../frontend/public/* public
	@mkdir -p images && cp -r ../frontend/images/* images
	@cp ../frontend/index.scss index.scss
	@trunk build index.html

run:
	@cargo run --features=ssr --bin server -- --dir dist
