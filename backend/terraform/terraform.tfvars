region = "eu-west-1"                   # <-- See https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.RegionsAndAvailabilityZones.html
domain = "ecc-website.net"             # <-- This should match your website domain
app-name = "ecc"                       # <-- Short but user-friendly app name
strava-client-id = "123456"            # <-- See https://developers.strava.com/docs/getting-started
strava-client-secret = "abc123def456"  # <-- as above
admin-strava-ids = "12345678,12345679" # <-- Strava IDs of any admins (comma-seperated inside the speech marks)
admin-emails = "name@email.com"        # <-- Email addresses that will recieve 'contact us' messages (comma-seperated inside the speech marks)
cert_arn = "arn:aws:acm:us-east-1:123456123456:certificate/a6f7e46c-0d13-4d9e-90d7-9b4e899f8cc9" # <-- AWS ARN of the domain certificate
