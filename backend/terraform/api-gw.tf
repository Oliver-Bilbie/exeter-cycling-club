resource "aws_apigatewayv2_api" "api_gw" {
  name          = "Exeter Cycling Club API"
  protocol_type = "HTTP"

  body = templatefile("${path.module}/../openapi.yaml", {
    auth_lambda_arn                    = aws_lambda_function.authenticate.invoke_arn,
    contact_lambda_arn                 = aws_lambda_function.contact.invoke_arn,
    get_route_lambda_arn               = aws_lambda_function.get_route.invoke_arn,
    set_route_lambda_arn               = aws_lambda_function.set_route.invoke_arn,
    cancel_route_lambda_arn            = aws_lambda_function.cancel_route.invoke_arn,
    email_subscribe_lambda_arn         = aws_lambda_function.email_subscribe.invoke_arn,
    email_confirm_subscribe_lambda_arn = aws_lambda_function.email_confirm_subscribe.invoke_arn,
    email_unsubscribe_lambda_arn       = aws_lambda_function.email_unsubscribe.invoke_arn,
    set_attendance_lambda_arn          = aws_lambda_function.set_attendance.invoke_arn,
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

  default_route_settings {
    throttling_rate_limit  = 10
    throttling_burst_limit = 100
  }
}

resource "aws_lambda_permission" "apigw_invoke" {
  for_each = {
    for fn in [
      aws_lambda_function.authenticate,
      aws_lambda_function.contact,
      aws_lambda_function.get_route,
      aws_lambda_function.set_route,
      aws_lambda_function.cancel_route,
      aws_lambda_function.email_subscribe,
      aws_lambda_function.email_confirm_subscribe,
      aws_lambda_function.email_unsubscribe,
      aws_lambda_function.set_attendance
    ] : fn.function_name => fn
  }

  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = each.value.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api_gw.execution_arn}/*"
}
