resource "aws_ssm_parameter" "strava_client_id_ssm" {
  name        = "${var.app-name}-strava-client-id"
  description = "Client ID for the ${var.app-name} Strava App"
  type        = "String"
  value       = var.strava-client-id
}

resource "aws_ssm_parameter" "strava_client_secret_ssm" {
  name        = "${var.app-name}-strava-client-secret"
  description = "Client secret for the ${var.app-name} Strava App"
  type        = "SecureString"
  value       = var.strava-client-secret
}

resource "aws_ssm_parameter" "admin_strava_ids_ssm" {
  name        = "${var.app-name}-admin-strava-ids"
  description = "Admin Strava IDs for the ${var.app-name} webapp"
  type        = "SecureString"
  value       = var.admin-strava-ids
}

resource "aws_ssm_parameter" "admin_emails_ssm" {
  name        = "${var.app-name}-admin-emails"
  description = "Admin email addresses for the ${var.app-name} webapp"
  type        = "SecureString"
  value       = var.admin-emails
}

resource "aws_ssm_parameter" "route_data" {
  name        = "${var.app-name}-route-data"
  description = "Route data for the ${var.app-name} webapp"
  type        = "String"
  value       = "{\"status\": \"unavailable\", \"message\": \"Subscribe to email updates to find out when a route is announced\", \"id\": \"\", \"name\": \"\", \"distance\": \"\", \"elevation_gain\": \"\", \"map_url\": \"\"}"

  lifecycle {
    ignore_changes = [value]
  }
}
