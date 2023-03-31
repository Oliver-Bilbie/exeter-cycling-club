resource "aws_ssm_parameter" "strava-client-id-ssm" {
  name        = "${var.app-name}-strava-client-id"
  description = "Client ID for the ${var.app-name} Strava App"
  type        = "String"
  value       = var.strava-client-id
}

resource "aws_ssm_parameter" "strava-client-secret-ssm" {
  name        = "${var.app-name}-strava-client-secret"
  description = "Client secret for the ${var.app-name} Strava App"
  type        = "SecureString"
  value       = var.strava-client-secret
}

resource "aws_ssm_parameter" "admin-strava-ids-ssm" {
  name        = "${var.app-name}-admin-strava-ids"
  description = "Admin Strava IDs for the ${var.app-name} webapp"
  type        = "SecureString"
  value       = var.admin-strava-ids
}

resource "aws_ssm_parameter" "admin-emails-ssm" {
  name        = "${var.app-name}-admin-emails"
  description = "Admin email addresses for the ${var.app-name} webapp"
  type        = "SecureString"
  value       = var.admin-emails
}
