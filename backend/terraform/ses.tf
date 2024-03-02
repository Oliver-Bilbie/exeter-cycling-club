resource "aws_ses_domain_identity" "ecc_domain_identity" {
  domain = var.domain
}
