from crypt import methods
from flask import Flask
import json

app = Flask(__name__)

@app.route("/")
def index():
    """Index Function"""
    return "Nothing to see here..."


# Getting the current User
@app.route("/user", methods=["GET"])
def get_user():
    """This function is used for accessing the cache and seeing if there is a currently logged in user"""

    with open("user.json", mode="r", encoding='utf-8') as user_file:
        data = json.load(user_file)

    if "access_token" in data:
        valid = True
        print('user found')
    else:
        valid = False
        username = ""
        print('user not found')

    return {
        "valid": valid,
        "username": username,
    }


# Setting the current User


# Setting the Wifi Credentials
