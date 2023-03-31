import os
import json
import requests


def authenticate_user(event, context):
    """
    Handles user authentication to Strava using OAuth 2.0 and returns some basic user data.
    See https://developers.strava.com/ for more information on the authentication process.
    """

    try:
        code = event.get("pathParameters").get("code")

        id, name, access_token = handle_authentication(code)
        admin = check_if_admin(id)

        user_data = {
            "id": id, "name": name, "access_token": access_token, "admin": admin
        }
        response = json.dumps({"status": 200, "body": user_data})

    except:
        response = json.dumps({"status": 500, "body": "Unable to authenticate"})

    return response


def handle_authentication(code):
    """
    Handles user authentication to Strava using OAuth 2.0 and returns some basic user data.

    Args:
        code [string]: Strava OAuth code

    Returns:
        id [string]: Strava ID of the authenticated user
        name [string]: Full name of the authenticated user
        access_token [string]: User authentication token
    """

    CLIENT_ID = os.getenv("CLIENT_ID", None)
    CLIENT_SECRET = os.getenv("CLIENT_SECRET", None)

    auth_response = requests.post(f"https://www.strava.com/oauth/token?client_id={CLIENT_ID}&client_secret={CLIENT_SECRET}&code={code}&grant_type=authorization_code")
    
    if auth_response.status_code == 200:
        auth_response = auth_response.json()
        id = str(auth_response.get("athlete").get("id"))
        name = f'{auth_response.get("athlete").get("firstname")} {auth_response.get("athlete").get("lastname")}'
        access_token = auth_response.get("access_token")
    
    else:
        raise Exception()

    return id, name, access_token


def check_if_admin(id):
    """
    Checks whether a user is an admin.

    Args:
        id [string]: Strava ID of the user

    Returns:
        [boolean]: True if the authenticated user is an admin
    """

    ADMIN_LIST = os.getenv("ADMIN_LIST", None)
    admins = ADMIN_LIST.split(",")

    return id in admins
