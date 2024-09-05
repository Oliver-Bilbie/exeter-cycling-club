default: build

build:
	@cd frontend && trunk build --release
	@cd frontend-ssr && make prd
	@cd backend && make

fmt:
	@cd backend && cargo fmt
	@cd frontend && cargo fmt
	@cd frontend-ssr && cargo fmt

update:
	@cd backend && cargo update
	@cd frontend && cargo update
	@cd frontend-ssr && cargo update
