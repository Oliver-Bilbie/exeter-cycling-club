resource "aws_lambda_function" "authenticate" {
  function_name    = "${var.app-name}-api-authenticate"
  filename         = "${path.module}/../target/lambda/ecc-api-authenticate/bootstrap.zip"
  source_code_hash = filesha256("${path.module}/../src/lambda/authenticate.rs")
  role             = aws_iam_role.authenticate_lambda_role.arn
  handler          = "bootstrap"
  runtime          = "provided.al2"
  timeout          = 20
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

resource "aws_lambda_function" "set_route" {
  function_name    = "${var.app-name}-api-set-route"
  filename         = "${path.module}/../target/lambda/ecc-api-set-route/bootstrap.zip"
  source_code_hash = filesha256("${path.module}/../src/lambda/set_route.rs")
  role             = aws_iam_role.set_route_lambda_role.arn
  handler          = "bootstrap"
  runtime          = "provided.al2"
  timeout          = 20
  memory_size      = 128

  environment {
    variables = {
      "TABLE_NAME"  = aws_dynamodb_table.mailing_list.id
      "BUCKET_NAME" = aws_s3_bucket.host_bucket.id
    }
  }
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
