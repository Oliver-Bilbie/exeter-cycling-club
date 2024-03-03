resource "aws_apigatewayv2_api" "api_gw" {
  name          = "Exeter Cycling Club API"
  protocol_type = "HTTP"
  body = templatefile("${path.module}/../openapi.yaml", {
    auth_lambda_arn              = aws_lambda_function.authenticate.invoke_arn,
    contact_lambda_arn           = aws_lambda_function.contact.invoke_arn,
    email_subscribe_lambda_arn   = aws_lambda_function.email_subscribe.invoke_arn,
    email_unsubscribe_lambda_arn = aws_lambda_function.email_unsubscribe.invoke_arn,
    # route_select_lambda_arn = aws_lambda_function.route_select.invoke_arn,
    # route_cancel_lambda_arn = aws_lambda_function.route_cancel.invoke_arn,
    # status_lambda_arn = aws_lambda_function.status.invoke_arn,
  })

  lifecycle {
    replace_triggered_by  = [terraform_data.deploy_api_gw]
  }
}

resource "aws_apigatewayv2_stage" "v1" {
  api_id      = aws_apigatewayv2_api.api_gw.id
  name        = "v1"
  auto_deploy = true
}

resource "terraform_data" "deploy_api_gw" {
  triggers_replace = filesha256("${path.module}/../openapi.yaml")
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
