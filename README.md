# Exeter Cycling Club Website

![Last-Commit](https://img.shields.io/github/last-commit/Oliver-Bilbie/exeter-cycling-club)

## Overview

Website for Exeter Cycling Club.
The web application allows users to authenticate using Strava's Oauth2. This allows club admins to create an upcoming ride based on a route from their Strava account. When a new route is set, members of the mailing list will be notified via email. These emails will give the member the option to confirm whether they are going to be coming on the ride - this attendance data is then sent to the club admins on the morning of the ride.

- Temporarily hosted [here](http://eccv2.oliver-bilbie.co.uk.s3-website-eu-west-1.amazonaws.com/) (*the site will remain HTTP only until its official release in order to avoid indexing*)

## To Do
### Frontend
- Add meta tags using the helmet API
- Refactor forms to only read the HTML state on submit

### Backend
- Configure CloudFront in front of the static hosting S3 bucket
- Configure a custom endpoint for the backend API Gateway
- Improve the HTML formatting of emails
