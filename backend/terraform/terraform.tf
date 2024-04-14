terraform {
  backend "s3" {
    region         = "eu-west-1"
    bucket         = "oliver-bilbie-tf-state-bucket"
    key            = "exeter-cycling-club/terraform.tfstate"
    dynamodb_table = "oliver-bilbie-tf-lock-table"
    encrypt        = true
  }
}
