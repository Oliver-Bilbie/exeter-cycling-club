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

        # Before updating the route we should confirm that the request was made by an admin
        check_admin_status(access_token)
        
        name, distance, elevation_gain, map_url = get_route_data(access_token, id)
        gpx = get_route_gpx(access_token, id)

        # Convert distances to a more readable format
        elevation_gain = str(elevation_gain).split(".")[0] + "m"
        if distance >= 1000:
            distance = str(distance / 1000).split(".")[0] + "km"
        else:
            distance = str(distance).split(".")[0] + "m"

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
    """
    Validate that the request was made by an admin by comparing the returned
    ID value to the list of admin IDs.

    Args:
        access_token [string]: Strava access token of the authenticated user
    """

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
    """
    Retrieves the route data from Strava for the specified route ID.
    
    Args:
        access_token [string]: Strava access token of the authenticated user
        id [string]: Strava route ID of the chosen route
    """
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
    """
    Retrieves the GPX file from Strava for the specified route ID.

    Args:
        access_token [string]: Strava access token of the authenticated user
        id [string]: Strava route ID of the chosen route
    """

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
    """
    Uploads a Python dictionary to the specified S3 bucket as a JSON file.

    Args:
        body [dict]: Dictionary to be uploaded
        key [string]: Name of the file once uploaded
    """

    S3_BUCKET = os.getenv("S3_BUCKET", None)
    s3_client = boto3.client("s3")

    with tempfile.NamedTemporaryFile() as output_file:
        output_file.write(str.encode(body))
        output_file.flush()
        s3_client.upload_file(output_file.name, S3_BUCKET, key)


def send_email_notifications(
    route_name, description, distance, elevation_gain, map_url
):
    """
    Generates the email body and sends the email to the admins.

    Args:
        route_name [string]: Name of the route
        description [string]: Description of the route
        distance [string]: Distance of the route
        elevation_gain [string]: Elevation gain of the route
        map_url [string]: URL of the route image
    """

    dynamodb_table = os.getenv("TABLE_NAME", None)
    ses_client = boto3.client("ses")
    dynamodb_client = boto3.client("dynamodb")

    mailing_list = dynamodb_client.scan(TableName=dynamodb_table).get("Items")

    
    for entry in mailing_list:
        SENDER = "Exeter Cycling Club <updates@oliver-bilbie.co.uk>"
        RECIPIENT = entry.get("email").get("S")
        RECIPIENT_ID = entry.get("id").get("S")
        SUBJECT = "This week's route"

        with open("src/templates/update.txt", "r") as text_file:
            BODY_TEXT = text_file.read()
        with open("src/templates/update.html", "r") as html_file:
            BODY_HTML = html_file.read()

        BODY_TEXT = BODY_TEXT.replace(r"%ROUTE_NAME%", route_name)
        BODY_TEXT = BODY_TEXT.replace(r"%DISTANCE%", distance)
        BODY_TEXT = BODY_TEXT.replace(r"%ELEVATION_GAIN%", elevation_gain)
        BODY_TEXT = BODY_TEXT.replace(r"%DESCRIPTION%", description.replace("$NEWLINE", "\n"))
        BODY_TEXT = BODY_TEXT.replace(r"%RECIPIENT_ID%", RECIPIENT_ID)

        BODY_HTML = BODY_HTML.replace(r"%ROUTE_NAME%", route_name)
        BODY_HTML = BODY_HTML.replace(r"%MAP_URL%", map_url)
        BODY_HTML = BODY_HTML.replace(r"%DISTANCE%", distance)
        BODY_HTML = BODY_HTML.replace(r"%ELEVATION_GAIN%", elevation_gain)
        BODY_HTML = BODY_HTML.replace(r"%DESCRIPTION%", description.replace("$NEWLINE", "<br />"))
        BODY_HTML = BODY_HTML.replace(r"%RECIPIENT_ID%", RECIPIENT_ID)

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

        ses_client.send_raw_email(
            Source=SENDER,
            Destinations=[RECIPIENT],
            RawMessage={
                "Data": msg.as_string(),
            },
        )
