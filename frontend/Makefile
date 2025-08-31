.DEFAULT_GOAL := build

build:
	@rm -rf public/bulma
	@cd public && git clone https://github.com/jgthms/bulma.git && cd bulma && git checkout 0.9.4
	@trunk build --release

dev:
	@trunk serve --open --port 8080
