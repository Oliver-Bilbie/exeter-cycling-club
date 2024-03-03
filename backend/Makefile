.DEFAULT_GOAL := all

all: validate build plan apply

build:
	@cargo lambda build --release --output-format zip

init:
	@cd terraform && terraform init -reconfigure

validate:
	@cd terraform && terraform validate

plan:
	@cd terraform && terraform plan -out=plan.out -var-file="./terraform.tfvars"

apply:
	@cd terraform && terraform apply -auto-approve -var-file="./terraform.tfvars"

destroy:
	@cd terraform && terraform destroy -auto-approve -var-file="./terraform.tfvars"
