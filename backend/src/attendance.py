import os
import json
import boto3
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from email.mime.application import MIMEApplication

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
    dynamodb_table = os.getenv("TABLE_NAME", None)

    ses_client = boto3.client("ses")
    dynamodb_client = boto3.client("dynamodb")

    mailing_list = os.getenv("ADMIN_EMAILS").split(",")
    newLineChar = "\n"  # necessary due to the limitations of f-strings in Python

    for admin_email in mailing_list:
        SENDER = "Exeter Cycling Club <updates@oliver-bilbie.co.uk>"
        RECIPIENT = admin_email
        SUBJECT = "This week's riders"
        BODY_TEXT = f"This email is not being displayed correctly.\n\nThis week's riders:\nComing: {yes_list}\nMaybe: {maybe_list}\n\nYou are receiving this email because you are registered as an Exeter Cycling Club administrator. If you no longer wish to receive these updates then please contact Ollie at oliverbilbie@tuta.io."
        BODY_HTML = f"""\
<html>
<body>
<table style="height: 187px; width: 100%; border-collapse: collapse; border-style: none; cellpadding=0; cellspacing=0; border=0">
<tbody>
<tr style="height: 73px;">
<td style="width: 15%; height: 73px;">&nbsp;</td>
<td style="width: 70%; height: 73px; text-align: center;">
<h1><strong>This week's riders</strong></h1>
</td>
<td style="width: 15%; height: 73px;">&nbsp;</td>
</tr>
<tr style="height: 18px;">
<td style="width: 15%; height: 18px;">&nbsp;</td>
<td style="width: 70%; height: 18px; text-align: center;"><h3><strong>Coming:</strong></h3></td>
<td style="width: 15%; height: 18px;">&nbsp;</td>
</tr>
<tr style="height: 18px;">
<td style="width: 15%; height: 18px;">&nbsp;</td>
<td style="width: 70%; height: 18px; text-align: center;">{newLineChar.join(map(str, yes_list))}</td>
<td style="width: 15%; height: 18px;">&nbsp;</td>
</tr>
<tr style="height: 18px;">
<td style="width: 15%; height: 18px;">&nbsp;</td>
<td style="width: 70%; height: 18px; text-align: center;"><h3><strong>Maybe:</strong></h3></td>
<td style="width: 15%; height: 18px;">&nbsp;</td>
</tr>
<tr style="height: 18px;">
<td style="width: 15%; height: 18px;">&nbsp;</td>
<td style="width: 70%; height: 18px; text-align: center;">{newLineChar.join(map(str, maybe_list))}</td>
<td style="width: 15%; height: 18px;">&nbsp;</td>
</tr>
<tr style="height: 30px;">
<td style="width: 15%; height: 30px;">&nbsp;</td>
<td style="width: 70%; text-align: center; height: 30px;">&nbsp;</td>
<td style="width: 15%; height: 30px;">&nbsp;</td>
</tr>
<tr style="height: 23px;">
<td style="width: 15%; height: 23px;">&nbsp;</td>
<td style="width: 70%; text-align: center; height: 23px;">You are receiving this email because you are registered as an Exeter Cycling Club administrator. If you no longer wish to receive these updates then please contact Ollie at oliverbilbie@tuta.io.</td>
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
