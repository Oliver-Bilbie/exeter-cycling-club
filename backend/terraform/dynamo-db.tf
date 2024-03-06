resource "aws_dynamodb_table" "mailing_list" {
  name           = "${var.app-name}-mailing-list"
  billing_mode   = "PROVISIONED"
  read_capacity  = 1
  write_capacity = 1
  hash_key       = "id"

  attribute {
    name = "id"
    type = "S"
  }

  attribute {
    name = "email"
    type = "S"
  }

  global_secondary_index {
    name               = "EmailIndex"
    hash_key           = "email"
    write_capacity     = 1
    read_capacity      = 1
    projection_type    = "ALL"
  }

  tags = {
    Name        = "${var.app-name}-mailing-list"
    Description = "Mailing list for ${var.app-name}"
    Environment = "production"
  }
}
