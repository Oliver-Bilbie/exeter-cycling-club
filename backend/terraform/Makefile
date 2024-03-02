init:
	@echo "[INFO] Removing cached terraform modules"
	@rm -Rf .terraform/modules
	@echo "[INFO] Initialiasing terraform"
	@terraform init -reconfigure

validate:
	@echo "[INFO] Validating terraform code"
	@terraform validate

plan:
	@echo "[INFO] Running a terraform plan"
	@terraform plan -out=plan.out -var-file="./terraform.tfvars"

apply:
	@echo "[INFO] Deploying the infrastructure"
	@terraform apply -auto-approve -var-file="./terraform.tfvars"

destroy:
	@echo "[INFO] Destroying the deployed infrastructure"
	@terraform destroy -auto-approve -var-file="./terraform.tfvars"
