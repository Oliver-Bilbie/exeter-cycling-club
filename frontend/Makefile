.DEFAULT_GOAL := all

all: clean build compress deploy

clean:
	@rm -rf dist/*

build:
	@trunk build --release

compress:
	@find dist -type f -name "*.css" -exec gzip -9 {} \;
	@find dist -type f -name "*.js" -exec gzip -9 {} \;
	@find dist -type f -name "*.wasm" -exec gzip -9 {} \;
	@find dist -type f -name "*.gz" -exec sh -c 'mv "$$1" "$${1%.gz}"' _ {} \;

deploy:
	@aws s3 sync dist s3://eccv2.oliver-bilbie.co.uk --delete --content-encoding gzip --exclude "*.html" --exclude "*.svg" --exclude "*.png" --exclude "*.jpg" --exclude "*.jpeg" --exclude "*.ico"
	@aws s3 sync dist s3://eccv2.oliver-bilbie.co.uk --delete --exclude "*" --include "*.html" --include "*.svg" --include "*.png" --include "*.jpg" --include "*.jpeg" --include "*.ico"
