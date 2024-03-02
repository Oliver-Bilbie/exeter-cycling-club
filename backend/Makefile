.DEFAULT_GOAL := all

all: build deploy

build:
	@cargo lambda build --release

deploy:
	@cargo lambda deploy ecc-api-contact
	@cargo lambda deploy ecc-api-subscribe
	@cargo lambda deploy ecc-api-unsubscribe
