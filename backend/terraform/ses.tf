resource "aws_ses_domain_identity" "ecc_domain_identity" {
  domain = var.domain
}

data "aws_route53_zone" "ecc_zone" {
  name = var.domain
}

resource "aws_route53_record" "ecc_ses_verification_record" {
  zone_id = data.aws_route53_zone.ecc_zone.zone_id
  name    = "_amazonses.${var.domain}"
  type    = "TXT"
  records = [aws_ses_domain_identity.ecc_domain_identity.verification_token]
  ttl     = "600"
}
