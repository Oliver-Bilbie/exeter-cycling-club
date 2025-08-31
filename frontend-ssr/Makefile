.DEFAULT_GOAL := dev

dev: build_wasm run

prd: build_wasm build

build_wasm:
	@rm -rf public images index.scss dist
	@mkdir -p public && cp -r ../frontend/public/* public
	@mkdir -p images && cp -r ../frontend/images/* images
	@cp ../frontend/index.scss index.scss
	@trunk build --release=true index.html

run:
	@cargo run --features=ssr --bin server -- --dir dist

build:
	@rm -rf target/lambda/server/dist target/lambda/server/bootstrap.zip
	@mkdir -p target/lambda/server
	@cp -r dist target/lambda/server
	@rm -rf target/lambda/server/dist/public/bulma/docs
	@rm -rf target/lambda/server/dist/public/bulma/test
	@rm -rf target/lambda/server/dist/public/bulma/.git
	@rm -rf target/lambda/server/dist/public/bulma/.github
	@cargo lambda build --release --features=ssr --bin server --
	@cd target/lambda/server && zip -r bootstrap.zip bootstrap dist
