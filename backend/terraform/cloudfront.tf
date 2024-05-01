resource "aws_lambda_function_url" "render_ui_url" {
  function_name      = aws_lambda_function.render_ui.function_name
  authorization_type = "NONE"
}

resource "aws_cloudfront_distribution" "render_ui_distribution" {
  enabled         = true
  is_ipv6_enabled = true
  price_class     = "PriceClass_100"
  # TODO: aliases         = [var.app-name]

  origin {
    domain_name = "${aws_lambda_function_url.render_ui_url.url_id}.lambda-url.${var.region}.on.aws"
    origin_id   = "${var.app-name}-render-ui-lambda"
    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  default_cache_behavior {
    target_origin_id       = "${var.app-name}-render-ui-lambda"
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    compress               = false
    viewer_protocol_policy = "redirect-to-https"
    cache_policy_id        = aws_cloudfront_cache_policy.render_ui_cache_policy.id
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    cloudfront_default_certificate = true
  }

  tags = {
    Name = "${var.app-name}-render-ui"
  }
}

resource "aws_cloudfront_cache_policy" "render_ui_cache_policy" {
  name        = "${var.app-name}-render-ui"
  comment     = "Cache policy for the render-ui function of the ${var.app-name} application"
  default_ttl = 86400
  max_ttl     = 604800
  min_ttl     = 21600

  parameters_in_cache_key_and_forwarded_to_origin {
    enable_accept_encoding_brotli = true
    enable_accept_encoding_gzip   = false
    cookies_config {
      cookie_behavior = "none"
    }
    headers_config {
      header_behavior = "none"
    }
    query_strings_config {
      query_string_behavior = "none"
    }
  }
}
