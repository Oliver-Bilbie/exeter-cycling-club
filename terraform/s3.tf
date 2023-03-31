resource "aws_s3_bucket" "host-bucket" {
  bucket = var.domain
}

resource "aws_s3_bucket_policy" "allow_access" {
  bucket = aws_s3_bucket.host-bucket.id
  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Sid" : "PublicReadGetObject",
        "Effect" : "Allow",
        "Principal" : "*",
        "Action" : "s3:GetObject",
        "Resource" : "${aws_s3_bucket.host-bucket.arn}/*"
      }
    ]
  })
}

resource "aws_s3_bucket_cors_configuration" "example" {
  bucket = aws_s3_bucket.host-bucket.id

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["GET"]
    allowed_origins = ["*"]
  }
}

resource "aws_s3_bucket_website_configuration" "website-config" {
  bucket = aws_s3_bucket.host-bucket.id
  index_document {
    suffix = "index.html"
  }
  error_document {
    key = "index.html"
  }
}
