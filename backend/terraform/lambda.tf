resource "aws_lambda_function" "authenticate" {
  function_name    = "${var.app-name}-api-authenticate"
  filename         = "${path.module}/../target/lambda/ecc-api-authenticate/bootstrap.zip"
  source_code_hash = filesha256("${path.module}/../src/lambda/authenticate.rs")
  role             = aws_iam_role.authenticate_lambda_role.arn
  handler          = "bootstrap"
  runtime          = "provided.al2"
  timeout          = 10
  memory_size      = 128
}

resource "aws_lambda_function" "contact" {
  function_name    = "${var.app-name}-api-contact"
  filename         = "${path.module}/../target/lambda/ecc-api-contact/bootstrap.zip"
  source_code_hash = filesha256("${path.module}/../src/lambda/contact.rs")
  role             = aws_iam_role.contact_lambda_role.arn
  handler          = "bootstrap"
  runtime          = "provided.al2"
  timeout          = 10
  memory_size      = 128
}

resource "aws_lambda_function" "email_subscribe" {
  function_name    = "${var.app-name}-api-email-subscribe"
  filename         = "${path.module}/../target/lambda/ecc-api-subscribe/bootstrap.zip"
  source_code_hash = filesha256("${path.module}/../src/lambda/subscribe.rs")
  role             = aws_iam_role.email_subscribe_lambda_role.arn
  handler          = "bootstrap"
  runtime          = "provided.al2"
  timeout          = 10
  memory_size      = 128
}

resource "aws_lambda_function" "email_unsubscribe" {
  function_name    = "${var.app-name}-api-email-unsubscribe"
  filename         = "${path.module}/../target/lambda/ecc-api-unsubscribe/bootstrap.zip"
  source_code_hash = filesha256("${path.module}/../src/lambda/unsubscribe.rs")
  role             = aws_iam_role.email_unsubscribe_lambda_role.arn
  handler          = "bootstrap"
  runtime          = "provided.al2"
  timeout          = 10
  memory_size      = 128
}
