resource "aws_scheduler_schedule" "clear_route" {
  name                = "${var.app-name}-clear-route"
  schedule_expression = "cron(0 12 ? * SUN *)"

  target {
    arn      = aws_lambda_function.clear_route.arn
    role_arn = aws_iam_role.eventbridge_execution_role.arn
  }

  flexible_time_window {
    mode = "OFF"
  }
}

resource "aws_lambda_permission" "eventbridge_invoke_clear_route" {
  statement_id  = "AllowEventbridgeInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.clear_route.function_name
  principal     = "scheduler.amazonaws.com"
  source_arn    = aws_scheduler_schedule.clear_route.arn
}

data "aws_iam_policy_document" "eventbridge_execution_policy" {
  statement {
    effect  = "Allow"
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["scheduler.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "eventbridge_execution_role" {
  name               = "${var.app-name}-eventbridge-execution-role"
  assume_role_policy = data.aws_iam_policy_document.eventbridge_execution_policy.json
}
