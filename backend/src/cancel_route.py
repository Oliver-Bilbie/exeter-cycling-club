import os
import json
import tempfile
import requests
import boto3
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from email.mime.application import MIMEApplication


def cancel_route(event, context):
    """
    User-initiated function to cancel a previously scheduled route.
    This will notify the mailing list with a provided message.
    """
    try:
        body = event.get("body")
        body = json.loads(body)
        access_token = body.get("access_token")
        message = body.get("message")

        check_admin_status(access_token)
            
        output_body = json.dumps({
            "status": "cancelled",
            "message": message
        })

        # Upload data to be consumed by the website
        upload_to_s3(output_body, "routeData.json")

        send_email_notifications(message)

        response = json.dumps({"status": 200, "body": "Route cancelled successfully"})

    except:
        response = json.dumps({"status": 500, "body": "Server error"})

    return response


def remove_route(event, context):
    """
    CRON-triggered function to remove the currently set route when it
    takes place in the past.
    This will NOT notify the mailing list.
    """
    output_body = json.dumps({
        "status": "unavailable",
        "message": "Subscribe to email updates to find out as soon as it goes live."
    })

    # Upload data to be consumed by the website
    upload_to_s3(output_body, "routeData.json")


def check_admin_status(access_token):
    # Before updating the route we should confirm that the request was made by an admin
    # To do this we call the athlete API and compare the returned ID value to the list of admin IDs

    admins = os.getenv("ADMIN_LIST", None).split(",")

    response = requests.get(f"https://www.strava.com/api/v3/athlete", headers={"Authorization": f"Bearer {access_token}"})
    if response.status_code == 200:
        response = response.json()
        id = str(response.get("id"))

        if not id in admins:
            raise Exception()

    else:
        raise Exception()


def upload_to_s3(body, key):
    S3_BUCKET = os.getenv("S3_BUCKET", None)
    s3_client = boto3.client('s3')

    with tempfile.NamedTemporaryFile() as output_file:
        output_file.write(str.encode(body))
        output_file.flush()
        s3_client.upload_file(output_file.name, S3_BUCKET, key)


def send_email_notifications(message):
    dynamodb_table = os.getenv("TABLE_NAME", None)

    ses_client = boto3.client('ses')
    dynamodb_client = boto3.client('dynamodb')

    mailing_list = dynamodb_client.scan(TableName=dynamodb_table).get("Items")

    newLineChar = "\n" # necessary due to the limitations of f-strings in Python

    for entry in mailing_list:
        SENDER = "Exeter Cycling Club <updates@oliver-bilbie.co.uk>"
        RECIPIENT = entry.get("email").get("S")
        RECIPIENT_ID = entry.get("id").get("S")
        SUBJECT = "Ride cancelled"
        BODY_TEXT = f"This week's ride has been cancelled.\n{message.replace('$NEWLINE', newLineChar)}\n\nPlease do not reply to this email. You are receiving this email because you signed up for alerts from Exeter Cycling Club. If you no longer wish to receive these updates then you may unsubscribe using the link below.\nhttp://ecc.oliver-bilbie.co.uk.s3-website-eu-west-1.amazonaws.com/unsubscribe?id={RECIPIENT_ID}"
        BODY_HTML = f"""\
<html>
<body>
<table style="height: 187px; width: 100%; border-collapse: collapse; border-style: none; cellpadding=0; cellspacing=0; border=0">
<tbody>
<tr style="height: 73px;">
<td style="width: 15%; height: 73px;">&nbsp;</td>
<td style="width: 70%; height: 73px; text-align: center;">
<h1><strong>This week's ride has been cancelled</strong></h1>
</td>
<td style="width: 15%; height: 73px;">&nbsp;</td>
</tr>
<tr style="height: 18px;">
<td style="width: 15%; height: 18px;">&nbsp;</td>
<td style="width: 70%; height: 18px; text-align: center;">{message.replace("$NEWLINE", "<br>")}</td>
<td style="width: 15%; height: 18px;">&nbsp;</td>
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

        msg = MIMEMultipart('mixed')
        msg['Subject'] = SUBJECT
        msg['From'] = SENDER
        msg['To'] = RECIPIENT

        msg_body = MIMEMultipart('alternative')
        textpart = MIMEText(BODY_TEXT.encode("utf-8"), 'plain', "utf-8")
        htmlpart = MIMEText(BODY_HTML.encode("utf-8"), 'html', "utf-8")

        msg_body.attach(textpart)
        msg_body.attach(htmlpart)
        msg.attach(msg_body)

        ses_response = ses_client.send_raw_email(
            Source=SENDER,
            Destinations=[RECIPIENT],
            RawMessage={
                'Data':msg.as_string(),
            }
        )
