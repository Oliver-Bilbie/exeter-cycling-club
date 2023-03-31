import os
import json
import boto3
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from email.mime.application import MIMEApplication


def contact_us(event, context):
    """
    Sends an email to all admins.

    Args:
        contact_email [string]: Email to reply to
        message [string]: Email body
    """

    try:
        body = event.get("body")
        body = json.loads(body)
        contact_email = body.get("contact_email")
        message = body.get("message")

        ses_client = boto3.client('ses')
        mailing_list = os.getenv("ADMIN_EMAILS", None).split(",")

        for admin_email in mailing_list:
            SENDER = "Exeter Cycling Club <contact@oliver-bilbie.co.uk>"
            RECIPIENT = admin_email
            SUBJECT = "Message for ECC"
            BODY_TEXT = f"Contact Email: {contact_email}\n\n{message}"

            msg = MIMEMultipart('mixed')
            msg['Subject'] = SUBJECT
            msg['From'] = SENDER
            msg['To'] = RECIPIENT

            msg_body = MIMEMultipart('alternative')
            textpart = MIMEText(BODY_TEXT.encode("utf-8"), 'plain', "utf-8")

            msg_body.attach(textpart)
            msg.attach(msg_body)

            ses_response = ses_client.send_raw_email(
                Source=SENDER,
                Destinations=[RECIPIENT],
                RawMessage={
                    'Data':msg.as_string(),
                }
            )

        response = json.dumps({"status": 200, "body": "Message sent successfully"})

    except:
        response = json.dumps({"status": 500, "body": "Server error"})

    return response
