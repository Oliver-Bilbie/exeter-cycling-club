"""
This module contains the functions required for users to notify the admins that
they will be attending the upcoming ride.
"""

import os
import json
import boto3
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText

ses_client = boto3.client("ses")
dynamodb_client = boto3.client("dynamodb")
dynamodb_table = os.getenv("TABLE_NAME", None)


def set_status(event, context):
    """
    Update the dynamo table to reflect the user's status

    Args:
        id [string]: UUID of the user to set the status of
        status [string]: Status to set
    """

    try:
        body = event.get("body")
        body = json.loads(body)
        id = body.get("id")
        status = body.get("status").upper()

        if not status in ["Y", "N", "M"]:
            raise Exception("Invalid status")

        dynamodb_client.update_item(
            TableName=dynamodb_table,
            Key={"id": {"S": id}},
            UpdateExpression="SET rideStatus = :status",
            ExpressionAttributeValues={":status": {"S": status}},
        )

        response = json.dumps({"status": 200, "body": "Status set successfully"})

    except:
        response = json.dumps({"status": 500, "body": "Unable to set status"})

    return response


def get_report(event, context):
    """
    Sends a report to admins containing the attendance status of all users that have set their status for the upcoming ride.
    The status table is then reset for the next ride.
    """

    member_list = dynamodb_client.scan(TableName=dynamodb_table).get("Items")
    yes_list = []
    maybe_list = []

    for member in member_list:
        id = member.get("id").get("S")
        name = member.get("name").get("S")
        status = member.get("rideStatus").get("S")

        if status == "Y":
            yes_list.append(name)
        elif status == "M":
            maybe_list.append(name)

        dynamodb_client.update_item(
            TableName=dynamodb_table,
            Key={"id": {"S": id}},
            UpdateExpression="SET rideStatus = :status",
            ExpressionAttributeValues={":status": {"S": "N"}},
        )

    send_email_notifications(yes_list, maybe_list)


def send_email_notifications(yes_list, maybe_list):
    """
    Generates the email body and sends the email to the admins.

    Args:
        yes_list (string[]): Names of users that have confirmed they will be attending
        maybe_list (string[]): Names of users that have confirmed they may be attending
    """

    mailing_list = os.getenv("ADMIN_EMAILS").split(",")

    yes_list = "\n".join(map(str, yes_list))
    maybe_list = "\n".join(map(str, maybe_list))

    with open("src/templates/attendance.txt", "r") as text_file:
        BODY_TEXT = text_file.read()
    with open("src/templates/attendance.html", "r") as html_file:
        BODY_HTML = html_file.read()

    BODY_TEXT = BODY_TEXT.replace("%YES_LIST%", yes_list)
    BODY_TEXT = BODY_TEXT.replace("%MAYBE_LIST%", maybe_list)
    BODY_HTML = BODY_HTML.replace("%YES_LIST%", yes_list)
    BODY_HTML = BODY_HTML.replace("%MAYBE_LIST%", maybe_list)

    for admin_email in mailing_list:
        msg = MIMEMultipart("mixed")
        msg["Subject"] = "This week's riders"
        msg["From"] = "Exeter Cycling Club <updates@oliver-bilbie.co.uk>"
        msg["To"] = admin_email

        msg_body = MIMEMultipart("alternative")
        textpart = MIMEText(BODY_TEXT.encode("utf-8"), "plain", "utf-8")
        htmlpart = MIMEText(BODY_HTML.encode("utf-8"), "html", "utf-8")

        msg_body.attach(textpart)
        msg_body.attach(htmlpart)
        msg.attach(msg_body)

        ses_client.send_raw_email(
            Source=msg["From"],
            Destinations=[msg["To"]],
            RawMessage={
                "Data": msg.as_string(),
            },
        )
