variable "region" {
  type        = string
  description = "Name of the AWS region to provision resources in"
}

variable "domain" {
  type        = string
  description = "Website domain name"
}

variable "app-name" {
  type        = string
  description = "Short but user-friendly app name"
}

variable "strava-client-id" {
  type        = string
  description = "Client ID for the corresponding Strava App"
}

variable "strava-client-secret" {
  type        = string
  description = "Client secret for the corresponding Strava App"
}

variable "admin-strava-ids" {
  type        = string
  description = "Strava IDs of any admins"
}

variable "admin-emails" {
  type        = string
  description = "Email addresses of any admins"
}
