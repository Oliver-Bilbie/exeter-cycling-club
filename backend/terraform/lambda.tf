resource "aws_lambda_function" "contact" {
  function_name = "${var.app-name}-api-contact"
  filename      = "${path.module}/../target/lambda/ecc-api-contact/bootstrap.zip"
  role          = aws_iam_role.contact_lambda_role.arn
  handler       = "bootstrap"
  runtime       = "provided.al2"
}

resource "aws_lambda_function" "email_subscribe" {
  function_name = "${var.app-name}-api-email-subscribe"
  filename      = "${path.module}/../target/lambda/ecc-api-subscribe/bootstrap.zip"
  role          = aws_iam_role.email_subscribe_lambda_role.arn
  handler       = "bootstrap"
  runtime       = "provided.al2"
}

resource "aws_lambda_function" "email_unsubscribe" {
  function_name = "${var.app-name}-api-email-unsubscribe"
  filename      = "${path.module}/../target/lambda/ecc-api-unsubscribe/bootstrap.zip"
  role          = aws_iam_role.email_unsubscribe_lambda_role.arn
  handler       = "bootstrap"
  runtime       = "provided.al2"
}
