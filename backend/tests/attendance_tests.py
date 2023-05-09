import json
from unittest import mock

from src import attendance


def test_set_status_success(mocker):
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
    test_event = {"body": '{"id": "12345", "status": "invalid"}'}

    mocker.patch.object(attendance.dynamodb_client, "update_item")

    expected_response = json.dumps({"status": 500, "body": "Unable to set status"})
    actual_response = attendance.set_status(test_event, None)

    assert actual_response == expected_response
    attendance.dynamodb_client.update_item.assert_not_called()


def test_set_status_exception(mocker):
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
