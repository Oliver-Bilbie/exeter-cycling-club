resource "aws_apigatewayv2_api" "api_gw" {
  name          = "ecc-api"
  protocol_type = "HTTP"
  body = templatefile("${path.module}/../openapi.yaml", {
    # auth_lambda_arn = aws_lambda_function.auth.invoke_arn,
    contact_lambda_arn           = aws_lambda_function.contact.invoke_arn,
    email_subscribe_lambda_arn   = aws_lambda_function.email_subscribe.invoke_arn,
    email_unsubscribe_lambda_arn = aws_lambda_function.email_unsubscribe.invoke_arn,
    # route_select_lambda_arn = aws_lambda_function.route_select.invoke_arn,
    # route_cancel_lambda_arn = aws_lambda_function.route_cancel.invoke_arn,
    # status_lambda_arn = aws_lambda_function.status.invoke_arn,
  })
}

resource "aws_apigatewayv2_stage" "v1" {
  api_id      = aws_apigatewayv2_api.api_gw.id
  name        = "v1"
  auto_deploy = true
}
