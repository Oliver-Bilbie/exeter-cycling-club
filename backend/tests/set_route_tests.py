import json
from unittest import mock

from src import set_route


def test_set_route_success(mocker):
    """Test the set_route function for a successful request"""

    test_event = {"body": '{"id": "12345", "access_token": "abcde", "description": "Example description"}'}

    mocker.patch.object(set_route, "check_admin_status")
    mocker.patch.object(set_route, "get_route_data")
    set_route.get_route_data.return_value = ("Example name", 80000, 1200, "https://example.com")
    mocker.patch.object(set_route, "get_route_gpx")
    set_route.get_route_gpx.return_value = "Example GPX"
    mocker.patch.object(set_route, "upload_to_s3")
    mocker.patch.object(set_route, "send_email_notifications")

    expected_response = json.dumps({"status": 200, "body": "Route set successfully"})
    actual_response = set_route.set_route(test_event, None)

    assert actual_response == expected_response

    set_route.check_admin_status.assert_called_once_with("abcde")
    set_route.get_route_data.assert_called_once_with("abcde", "12345")
    set_route.get_route_gpx.assert_called_once_with("abcde", "12345")
    set_route.upload_to_s3.assert_has_calls(
        [
            mock.call(
                '{"status": "ready", "id": "12345", "name": "Example name", "distance": "80km", "elevation_gain": "1200m", "map_url": "https://example.com", "description": "Example description", "gpx": "Example GPX"}',
                "routeData.json",
            ),
            mock.call("Example GPX", "eccRoute.gpx"),
        ]
    )
    set_route.send_email_notifications.assert_called_once_with(
        "Example name", "Example description", "80km", "1200m", "https://example.com"
    )


def test_set_route_success_small_distance(mocker):
    """Test the set_route function for a successful request where the distance cannot be converted to km"""

    test_event = {"body": '{"id": "12345", "access_token": "abcde", "description": "Example description"}'}

    mocker.patch.object(set_route, "check_admin_status")
    mocker.patch.object(set_route, "get_route_data")
    set_route.get_route_data.return_value = ("Example name", 150, 100, "https://example.com")
    mocker.patch.object(set_route, "get_route_gpx")
    set_route.get_route_gpx.return_value = "Example GPX"
    mocker.patch.object(set_route, "upload_to_s3")
    mocker.patch.object(set_route, "send_email_notifications")

    expected_response = json.dumps({"status": 200, "body": "Route set successfully"})
    actual_response = set_route.set_route(test_event, None)

    assert actual_response == expected_response

    set_route.check_admin_status.assert_called_once_with("abcde")
    set_route.get_route_data.assert_called_once_with("abcde", "12345")
    set_route.get_route_gpx.assert_called_once_with("abcde", "12345")
    set_route.upload_to_s3.assert_has_calls(
        [
            mock.call(
                '{"status": "ready", "id": "12345", "name": "Example name", "distance": "150m", "elevation_gain": "100m", "map_url": "https://example.com", "description": "Example description", "gpx": "Example GPX"}',
                "routeData.json",
            ),
            mock.call("Example GPX", "eccRoute.gpx"),
        ]
    )
    set_route.send_email_notifications.assert_called_once_with(
        "Example name", "Example description", "150m", "100m", "https://example.com"
    )
