import os
import json
import uuid
import boto3

ses_client = boto3.client("ses")
dynamodb_client = boto3.client("dynamodb")
dynamodb_table = os.getenv("TABLE_NAME", None)


def subscribe(event, context):
    """
    Adds an email address to the mailing list
    """

    try:
        body = event.get("body")
        body = json.loads(body)
        name = body.get("name")
        email = body.get("email").lower()

        # Check that the email address is not already on the mailing list
        if (
            dynamodb_client.query(
                TableName=dynamodb_table,
                IndexName="EmailIndex",
                KeyConditionExpression="#em = :em",
                ExpressionAttributeNames={"#em":"email"},
                ExpressionAttributeValues={":em": {"S": email}}
            )["Count"]
            == 0
        ):
            # Send email verification request
            ses_client.verify_email_identity(EmailAddress=email)

            # Add email to DynamoDB table
            dynamodb_client.put_item(
                TableName=dynamodb_table,
                Item={
                    "id": {"S": str(uuid.uuid4())},
                    "name": {"S": name},
                    "email": {"S": email},
                    "verified": {"BOOL": False},
                    "bounces": {"N": "0"},
                },
            )

        response = json.dumps({"status": 200, "body": "Subscribed successfully"})

    except:
        response = json.dumps({"status": 500, "body": "Unable to subscribe"})

    return response


def unsubscribe(event, context):
    """
    Removes an email address to the mailing list
    """

    try:
        body = event.get("body")
        body = json.loads(body)
        id = body.get("id")
        email = (
            dynamodb_client.get_item(TableName=dynamodb_table, Key={"id": {"S": id}})
            .get("Item")
            .get("email")
            .get("S")
        )

        # Remove email from DynamoDB table
        dynamodb_client.delete_item(TableName=dynamodb_table, Key={"id": {"S": id}})

        # Delete email verification status from SES
        ses_client.delete_identity(Identity=email)

        response = json.dumps({"status": 200, "body": "Unsubscribed successfully"})

    except:
        response = json.dumps({"status": 500, "body": "Unable to unsubscribe"})

    return response
