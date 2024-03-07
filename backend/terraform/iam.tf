resource "aws_iam_role" "authenticate_lambda_role" {
  name               = "${var.app-name}-authenticate-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json

  inline_policy {
    name   = "${var.app-name}-authenticate-lambda-policy"
    policy = data.aws_iam_policy_document.authenticate_lambda_policy.json
  }

  inline_policy {
    name   = "${var.app-name}-lambda-logging"
    policy = data.aws_iam_policy_document.lambda_logging.json
  }
}

resource "aws_iam_role" "contact_lambda_role" {
  name               = "${var.app-name}-contact-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json

  inline_policy {
    name   = "${var.app-name}-contact-lambda-policy"
    policy = data.aws_iam_policy_document.contact_lambda_policy.json
  }

  inline_policy {
    name   = "${var.app-name}-lambda-logging"
    policy = data.aws_iam_policy_document.lambda_logging.json
  }
}

resource "aws_iam_role" "set_route_lambda_role" {
  name               = "${var.app-name}-set-route-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json

  inline_policy {
    name   = "${var.app-name}-set-route-lambda-policy"
    policy = data.aws_iam_policy_document.set_route_lambda_policy.json
  }

  inline_policy {
    name   = "${var.app-name}-lambda-logging"
    policy = data.aws_iam_policy_document.lambda_logging.json
  }
}

resource "aws_iam_role" "get_route_lambda_role" {
  name               = "${var.app-name}-get-route-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json
  inline_policy {
    name   = "${var.app-name}-get-route-lambda-policy"
    policy = data.aws_iam_policy_document.get_route_lambda_policy.json
  }
  inline_policy {
    name   = "${var.app-name}-lambda-logging"
    policy = data.aws_iam_policy_document.lambda_logging.json
  }
}

resource "aws_iam_role" "email_subscribe_lambda_role" {
  name               = "${var.app-name}-email-subscribe-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json
  inline_policy {
    name   = "${var.app-name}-subscribe-lambda-policy"
    policy = data.aws_iam_policy_document.subscribe_lambda_policy.json
  }
  inline_policy {
    name   = "${var.app-name}-lambda-logging"
    policy = data.aws_iam_policy_document.lambda_logging.json
  }
}

resource "aws_iam_role" "email_unsubscribe_lambda_role" {
  name               = "${var.app-name}-email-unsubscribe-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json
  inline_policy {
    name   = "${var.app-name}-unsubscribe-lambda-policy"
    policy = data.aws_iam_policy_document.unsubscribe_lambda_policy.json
  }
  inline_policy {
    name   = "${var.app-name}-lambda-logging"
    policy = data.aws_iam_policy_document.lambda_logging.json
  }
}

data "aws_iam_policy_document" "assume_role_policy" {
  statement {
    effect  = "Allow"
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

data "aws_iam_policy_document" "lambda_logging" {
  statement {
    effect = "Allow"

    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:PutLogEvents",
    ]

    resources = ["arn:aws:logs:*:*:*"]
  }
}

data "aws_iam_policy_document" "authenticate_lambda_policy" {
  statement {
    effect  = "Allow"
    actions = ["ssm:GetParameters"]
    resources = [
      aws_ssm_parameter.strava_client_id_ssm.arn,
      aws_ssm_parameter.strava_client_secret_ssm.arn
    ]
  }

  statement {
    effect    = "Allow"
    actions   = ["ssm:GetParameter"]
    resources = [aws_ssm_parameter.admin_strava_ids_ssm.arn]
  }
}

data "aws_iam_policy_document" "contact_lambda_policy" {
  statement {
    effect    = "Allow"
    actions   = ["ssm:GetParameter"]
    resources = [aws_ssm_parameter.admin_emails_ssm.arn]
  }
  statement {
    effect    = "Allow"
    actions   = ["ses:SendEmail"]
    resources = ["*"]
  }
}

data "aws_iam_policy_document" "set_route_lambda_policy" {
  statement {
    effect    = "Allow"
    actions   = ["ssm:GetParameter"]
    resources = [aws_ssm_parameter.admin_strava_ids_ssm.arn]
  }
  statement {
    effect    = "Allow"
    actions   = ["ssm:PutParameter"]
    resources = [aws_ssm_parameter.route_data.arn]
  }
  statement {
    effect    = "Allow"
    actions   = ["dynamodb:Scan"]
    resources = [aws_dynamodb_table.mailing_list.arn]
  }
  statement {
    effect    = "Allow"
    actions   = ["ses:SendEmail"]
    resources = ["*"]
  }
}

data "aws_iam_policy_document" "get_route_lambda_policy" {
  statement {
    effect    = "Allow"
    actions   = ["ssm:GetParameter"]
    resources = [aws_ssm_parameter.route_data.arn]
  }
}

data "aws_iam_policy_document" "subscribe_lambda_policy" {
  statement {
    effect    = "Allow"
    actions   = ["dynamodb:Query"]
    resources = ["${aws_dynamodb_table.mailing_list.arn}/index/EmailIndex"]
  }
  statement {
    effect    = "Allow"
    actions   = ["dynamodb:PutItem"]
    resources = [aws_dynamodb_table.mailing_list.arn]
  }
  statement {
    effect    = "Allow"
    actions   = ["ses:CreateEmailIdentity"]
    resources = ["*"]
  }
}

data "aws_iam_policy_document" "unsubscribe_lambda_policy" {
  statement {
    effect    = "Allow"
    actions   = ["dynamodb:GetItem", "dynamodb:DeleteItem"]
    resources = [aws_dynamodb_table.mailing_list.arn]
  }
  statement {
    effect    = "Allow"
    actions   = ["ses:DeleteEmailIdentity"]
    resources = ["*"]
  }
}
