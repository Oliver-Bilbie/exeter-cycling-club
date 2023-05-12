import json
from unittest import mock

from src import attendance


def test_set_status_success(mocker):
    """Test the set_status function for a successful request"""

    test_event = {"body": '{"id": "12345", "status": "y"}'}

    mocker.patch.object(attendance.dynamodb_client, "update_item")

    expected_response = json.dumps({"status": 200, "body": "Status set successfully"})
    actual_response = attendance.set_status(test_event, None)

    assert actual_response == expected_response
    attendance.dynamodb_client.update_item.assert_called_once_with(
        TableName=None,
        Key={"id": {"S": "12345"}},
        UpdateExpression="SET rideStatus = :status",
        ExpressionAttributeValues={":status": {"S": "Y"}},
    )


def test_set_status_invalid(mocker):
    """Test the set_status function for an invalid request"""

    test_event = {"body": '{"id": "12345", "status": "invalid"}'}

    mocker.patch.object(attendance.dynamodb_client, "update_item")

    expected_response = json.dumps({"status": 500, "body": "Unable to set status"})
    actual_response = attendance.set_status(test_event, None)

    assert actual_response == expected_response
    attendance.dynamodb_client.update_item.assert_not_called()


def test_set_status_exception(mocker):
    """Test the set_status function for a request where an unexpected error occurs"""

    test_event = {"body": '{"id": "12345", "status": "N"}'}

    mocker.patch.object(attendance.dynamodb_client, "update_item")
    attendance.dynamodb_client.update_item.side_effect = Exception()

    expected_response = json.dumps({"status": 500, "body": "Unable to set status"})
    actual_response = attendance.set_status(test_event, None)

    assert actual_response == expected_response
    attendance.dynamodb_client.update_item.assert_called_once_with(
        TableName=None,
        Key={"id": {"S": "12345"}},
        UpdateExpression="SET rideStatus = :status",
        ExpressionAttributeValues={":status": {"S": "N"}},
    )


def test_get_report(mocker):
    """Test the get_report function for a successful request"""

    mocker.patch.object(attendance.dynamodb_client, "scan")
    attendance.dynamodb_client.scan.return_value = {
        "Items": [
            {"id": {"S": "12345"}, "name": {"S": "Test User"}, "rideStatus": {"S": "Y"}},
            {"id": {"S": "23456"}, "name": {"S": "Test User 2"}, "rideStatus": {"S": "M"}},
            {"id": {"S": "34567"}, "name": {"S": "Test User 3"}, "rideStatus": {"S": "N"}},
            {"id": {"S": "45678"}, "name": {"S": "Test User 4"}, "rideStatus": {"S": "Y"}},
        ]
    }
    mocker.patch.object(attendance.dynamodb_client, "update_item")
    mocker.patch.object(attendance, "send_email_notifications")

    attendance.get_report(None, None)

    attendance.dynamodb_client.update_item.assert_has_calls(
        [
            mock.call(
                TableName=None,
                Key={"id": {"S": "12345"}},
                UpdateExpression="SET rideStatus = :status",
                ExpressionAttributeValues={":status": {"S": "N"}},
            ),
            mock.call(
                TableName=None,
                Key={"id": {"S": "23456"}},
                UpdateExpression="SET rideStatus = :status",
                ExpressionAttributeValues={":status": {"S": "N"}},
            ),
            mock.call(
                TableName=None,
                Key={"id": {"S": "34567"}},
                UpdateExpression="SET rideStatus = :status",
                ExpressionAttributeValues={":status": {"S": "N"}},
            ),
            mock.call(
                TableName=None,
                Key={"id": {"S": "45678"}},
                UpdateExpression="SET rideStatus = :status",
                ExpressionAttributeValues={":status": {"S": "N"}},
            ),
        ]
    )

    attendance.send_email_notifications.assert_called_once_with(
        ["Test User", "Test User 4"], ["Test User 2"]
    )


def test_send_email_notifications(mocker):
    """Test the send_email_notifications function for a successful request"""

    mocker.patch.object(attendance.os, "getenv")
    attendance.os.getenv.return_value = "admin1@email.com,admin2@email.com"
    mocker.patch.object(attendance, "open")
    attendance.open.side_effect = mock.mock_open(read_data="Email body text").return_value
    attendance.body_text = "Email body text"
    mocker.patch.object(attendance.ses_client, "send_raw_email")

    yes_list = ["Test User", "Test User 2"]
    maybe_list = ["Test User 3"]


    attendance.send_email_notifications(yes_list, maybe_list)

    attendance.ses_client.send_raw_email.assert_has_calls(
        [
            mock.call(
                Source="Exeter Cycling Club <updates@oliver-bilbie.co.uk>",
                Destinations=["admin1@email.com"],
                RawMessage={
                    "Data": mock.ANY
                }
            ),
            mock.call(
                Source="Exeter Cycling Club <updates@oliver-bilbie.co.uk>",
                Destinations=["admin2@email.com"],
                RawMessage={
                    "Data": mock.ANY
                }
            ),
        ]
    )
