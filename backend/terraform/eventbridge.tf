resource "aws_cloudwatch_event_rule" "trigger_clear_route" {
  name                = "${var.app-name}-trigger-clear-route"
  description         = "Automatically trigger the ${var.app-name} clear_route lambda function at 12pm every Sunday"
  schedule_expression = "cron(0 12 ? * SUN *)"
}

resource "aws_cloudwatch_event_target" "trigger_clear_route" {
  rule = aws_cloudwatch_event_rule.trigger_clear_route.name
  arn  = aws_lambda_function.clear_route.arn
}

resource "aws_lambda_permission" "allow_trigger_clear_route" {
  statement_id  = "AllowExecutionFromCloudWatch"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.clear_route.function_name
  principal     = "events.amazonaws.com"
  source_arn    = aws_cloudwatch_event_rule.trigger_clear_route.arn
}

resource "aws_cloudwatch_event_rule" "trigger_attendance_report" {
  name                = "${var.app-name}-trigger-attendance-report"
  description         = "Automatically trigger the ${var.app-name} attendance_report lambda function at 5am every Sunday"
  schedule_expression = "cron(0 5 ? * SUN *)"
}

resource "aws_cloudwatch_event_target" "trigger_attendance_report" {
  rule = aws_cloudwatch_event_rule.trigger_attendance_report.name
  arn  = aws_lambda_function.attendance_report.arn
}

resource "aws_lambda_permission" "allow_trigger_attendance_report" {
  statement_id  = "AllowExecutionFromCloudWatch"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.attendance_report.function_name
  principal     = "events.amazonaws.com"
  source_arn    = aws_cloudwatch_event_rule.trigger_attendance_report.arn
}
