.DEFAULT_GOAL := all

all: build deploy

build:
	@trunk build --release

deploy:
	@aws s3 sync dist s3://eccv2.oliver-bilbie.co.uk --delete
