resource "aws_apigatewayv2_api" "api_gw" {
  name          = "Exeter Cycling Club API"
  protocol_type = "HTTP"

  body = templatefile("${path.module}/../openapi.yaml", {
    auth_lambda_arn              = aws_lambda_function.authenticate.invoke_arn,
    contact_lambda_arn           = aws_lambda_function.contact.invoke_arn,
    set_route_lambda_arn         = aws_lambda_function.set_route.invoke_arn,
    get_route_lambda_arn         = aws_lambda_function.get_route.invoke_arn,
    email_subscribe_lambda_arn   = aws_lambda_function.email_subscribe.invoke_arn,
    email_unsubscribe_lambda_arn = aws_lambda_function.email_unsubscribe.invoke_arn,
    # route_cancel_lambda_arn = aws_lambda_function.route_cancel.invoke_arn,
    # status_lambda_arn = aws_lambda_function.status.invoke_arn,
  })

  cors_configuration {
    allow_origins = ["*"]
    allow_methods = ["*"]
    allow_headers = ["*"]
  }
}

resource "aws_apigatewayv2_stage" "v1" {
  api_id      = aws_apigatewayv2_api.api_gw.id
  name        = "v1"
  auto_deploy = true
}

resource "aws_lambda_permission" "apigw_invoke_authenticate" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.authenticate.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api_gw.execution_arn}/*"
}

resource "aws_lambda_permission" "apigw_invoke_contact" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.contact.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api_gw.execution_arn}/*"
}

resource "aws_lambda_permission" "apigw_invoke_set_route" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.set_route.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api_gw.execution_arn}/*"
}

resource "aws_lambda_permission" "apigw_invoke_get_route" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.get_route.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api_gw.execution_arn}/*"
}

resource "aws_lambda_permission" "apigw_invoke_email_subscribe" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.email_subscribe.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api_gw.execution_arn}/*"
}

resource "aws_lambda_permission" "apigw_invoke_email_unsubscribe" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.email_unsubscribe.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api_gw.execution_arn}/*"
}
