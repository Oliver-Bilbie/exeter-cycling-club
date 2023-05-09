"""
This module contains the functions required for admins to set the upcoming
route. Users that have registered for email updates will be notified.
"""

import os
import json
import tempfile
import requests
import boto3
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from email.mime.application import MIMEApplication


def set_route(event, context):
    """
    Retrieves route data from Strava and writes it to S3.

    Args:
        access_token [string]: Strava access token of the authenticated user
        id [string]: Strava ID of the chosen route
    """

    try:
        body = event.get("body")
        body = json.loads(body)
        access_token = body.get("access_token")
        id = body.get("id")
        description = body.get("description")

        check_admin_status(access_token)
        name, distance, elevation_gain, map_url = get_route_data(access_token, id)
        gpx = get_route_gpx(access_token, id)

        # Convert distances to a more readable format
        distance = str(distance).split(".")[0][:-3] + "km"
        elevation_gain = str(elevation_gain).split(".")[0] + "m"

        output_body = json.dumps(
            {
                "status": "ready",
                "id": id,
                "name": name,
                "distance": distance,
                "elevation_gain": elevation_gain,
                "map_url": map_url,
                "description": description,
                "gpx": gpx,
            }
        )

        # Upload data to be consumed by the website
        upload_to_s3(output_body, "routeData.json")

        # Upload GPX to be downloaded by users
        upload_to_s3(gpx, "eccRoute.gpx")

        send_email_notifications(name, description, distance, elevation_gain, map_url)

        response = json.dumps({"status": 200, "body": "Route set successfully"})

    except:
        response = json.dumps({"status": 500, "body": "Server error"})

    return response


def check_admin_status(access_token):
    # Before updating the route we should confirm that the request was made by an admin
    # To do this we call the athlete API and compare the returned ID value to the list of admin IDs

    admins = os.getenv("ADMIN_LIST").split(",")

    response = requests.get(
        f"https://www.strava.com/api/v3/athlete",
        headers={"Authorization": f"Bearer {access_token}"},
    )
    if response.status_code == 200:
        response = response.json()
        id = str(response.get("id"))

        if not id in admins:
            raise Exception()

    else:
        raise Exception()


def get_route_data(access_token, id):
    response = requests.get(
        f"https://www.strava.com/api/v3/routes/{id}",
        headers={"Authorization": f"Bearer {access_token}"},
    )

    if response.status_code == 200:
        response = response.json()
        name = response.get("name")
        distance = response.get("distance")
        elevation_gain = response.get("elevation_gain")
        map_url = response.get("map_urls").get("retina_url")

    else:
        raise Exception()

    return name, distance, elevation_gain, map_url


def get_route_gpx(access_token, id):
    response = requests.get(
        f"https://www.strava.com/api/v3/routes/{id}/export_gpx",
        headers={"Authorization": f"Bearer {access_token}"},
    )

    if response.status_code == 200:
        gpx = response.content.decode("utf-8")

    else:
        raise Exception()

    return gpx


def upload_to_s3(body, key):
    S3_BUCKET = os.getenv("S3_BUCKET", None)
    s3_client = boto3.client("s3")

    with tempfile.NamedTemporaryFile() as output_file:
        output_file.write(str.encode(body))
        output_file.flush()
        s3_client.upload_file(output_file.name, S3_BUCKET, key)


def send_email_notifications(
    route_name, description, distance, elevation_gain, map_url
):
    dynamodb_table = os.getenv("TABLE_NAME", None)

    ses_client = boto3.client("ses")
    dynamodb_client = boto3.client("dynamodb")

    mailing_list = dynamodb_client.scan(TableName=dynamodb_table).get("Items")

    newLineChar = "\n"  # necessary due to the limitations of f-strings in Python

    for entry in mailing_list:
        SENDER = "Exeter Cycling Club <updates@oliver-bilbie.co.uk>"
        RECIPIENT = entry.get("email").get("S")
        RECIPIENT_ID = entry.get("id").get("S")
        SUBJECT = "This week's route"
        BODY_TEXT = f"This email is not being displayed correctly.\nIf you prefer you can view the route on our website instead at http://ecc.oliver-bilbie.co.uk.s3-website-eu-west-1.amazonaws.com/upcoming\n\nThis week's route:\n{route_name}\nDistance:{distance} - Elevation: {elevation_gain}\n{description.replace('$NEWLINE', newLineChar)}\n\nPlease do not reply to this email. You are receiving this email because you signed up for alerts from Exeter Cycling Club. If you no longer wish to receive these updates then you may unsubscribe using the link below.\nhttp://ecc.oliver-bilbie.co.uk.s3-website-eu-west-1.amazonaws.com/unsubscribe?id={RECIPIENT_ID}"
        BODY_HTML = f"""\
<html>
<body>
<table style="height: 187px; width: 100%; border-collapse: collapse; border-style: none; cellpadding=0; cellspacing=0; border=0">
<tbody>
<tr style="height: 73px;">
<td style="width: 15%; height: 73px;">&nbsp;</td>
<td style="width: 70%; height: 73px; text-align: center;">
<h1><strong>{route_name}</strong></h1>
</td>
<td style="width: 15%; height: 73px;">&nbsp;</td>
</tr>
<tr style="height: 18px;">
<td style="width: 15%; height: 18px;">&nbsp;</td>
<td style="width: 70%; height: 18px; text-align: center;"><img src="{map_url}" alt="Strava Map" width="453" height="189" /></td>
<td style="width: 15%; height: 18px;">&nbsp;</td>
</tr>
<tr style="height: 18px;">
<td style="width: 15%; height: 55px;">&nbsp;</td>
<td style="width: 70%; height: 55px; text-align: center;">
<h3><strong>Distance:</strong> {distance} - <strong>Elevation:</strong> {elevation_gain}</h3>
</td>
<td style="width: 15%; height: 55px;">&nbsp;</td>
</tr>
<tr style="height: 18px;">
<td style="width: 15%; height: 18px;">&nbsp;</td>
<td style="width: 70%; height: 18px; text-align: center;">{description.replace("$NEWLINE", "<br>")}</td>
<td style="width: 15%; height: 18px;">&nbsp;</td>
</tr>
<tr style="height: 23px;">
<td style="width: 15%; height: 23px;">&nbsp;</td>
<td style="width: 70%; text-align: center; height: 23px;"><a title="View" href="http://ecc.oliver-bilbie.co.uk.s3-website-eu-west-1.amazonaws.com/upcoming" target="_blank">View on our website</a></td>
<td style="width: 15%; height: 23px;">&nbsp;</td>
</tr>
<tr style="height: 30px;">
<td style="width: 15%; height: 30px;">&nbsp;</td>
<td style="width: 70%; text-align: center; height: 30px;">&nbsp;</td>
<td style="width: 15%; height: 30px;">&nbsp;</td>
</tr>
<tr style="height: 73px;">
<td style="width: 15%; height: 73px;">&nbsp;</td>
<td style="width: 70%; height: 73px; text-align: center;">
<h2><strong>Let us know you're coming</strong></h2>
</td>
<td style="width: 15%; height: 73px;">&nbsp;</td>
</tr>
<tr style="height: 23px;">
<td style="width: 15%; height: 23px;">&nbsp;</td>
<td style="width: 70%; text-align: center; height: 23px;"><a title="YesStatus" href="http://ecc.oliver-bilbie.co.uk.s3-website-eu-west-1.amazonaws.com/status?id={RECIPIENT_ID}&status=Y" target="_blank">I'll be there!</a></td>
<td style="width: 15%; height: 23px;">&nbsp;</td>
</tr>
<tr style="height: 23px;">
<td style="width: 15%; height: 23px;">&nbsp;</td>
<td style="width: 70%; text-align: center; height: 23px;"><a title="MaybeStatus" href="http://ecc.oliver-bilbie.co.uk.s3-website-eu-west-1.amazonaws.com/status?id={RECIPIENT_ID}&status=M" target="_blank">Maybe</a></td>
<td style="width: 15%; height: 23px;">&nbsp;</td>
</tr>
<tr style="height: 30px;">
<td style="width: 15%; height: 30px;">&nbsp;</td>
<td style="width: 70%; text-align: center; height: 30px;">&nbsp;</td>
<td style="width: 15%; height: 30px;">&nbsp;</td>
</tr>
<tr style="height: 23px;">
<td style="width: 15%; height: 23px;">&nbsp;</td>
<td style="width: 70%; text-align: center; height: 23px;">Please do not reply to this email. You are receiving this email because you signed up for alerts from Exeter Cycling Club. If you no longer wish to receive these updates then you may unsubscribe using the link below.</td>
<td style="width: 15%; height: 23px;">&nbsp;</td>
</tr>
<tr style="height: 23px;">
<td style="width: 15%; height: 23px;">&nbsp;</td>
<td style="width: 70%; text-align: center; height: 23px;"><a title="Unsubscribe" href="http://ecc.oliver-bilbie.co.uk.s3-website-eu-west-1.amazonaws.com/unsubscribe?id={RECIPIENT_ID}" target="_blank">Unsubscribe</a></td>
<td style="width: 15%; height: 23px;">&nbsp;</td>
</tr>
</tbody>
</table>
</body>
</html>
    """

        msg = MIMEMultipart("mixed")
        msg["Subject"] = SUBJECT
        msg["From"] = SENDER
        msg["To"] = RECIPIENT

        msg_body = MIMEMultipart("alternative")
        textpart = MIMEText(BODY_TEXT.encode("utf-8"), "plain", "utf-8")
        htmlpart = MIMEText(BODY_HTML.encode("utf-8"), "html", "utf-8")

        msg_body.attach(textpart)
        msg_body.attach(htmlpart)
        msg.attach(msg_body)

        ses_response = ses_client.send_raw_email(
            Source=SENDER,
            Destinations=[RECIPIENT],
            RawMessage={
                "Data": msg.as_string(),
            },
        )
