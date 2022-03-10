import sys
from flask import Flask, request, redirect
from flask_cors import CORS
import json
from dotenv import dotenv_values

from spotapi import generate_spotify_handler, load_spotify_handler



app = Flask(__name__)
CORS(app)
app.run(debug=True)

@app.route("/", methods=["GET"])
def index():
    """Index Function"""

    return "Nothing to see here..."

@app.route("/alive", methods=["GET"])
def alive():
    """Just a proof of life endpoint to check to see if the backend is up and running"""

    return "alive"

# Callback for the "code" value
@app.route("/spotify-login", methods=["GET"])
def code_login():
    """The callback run when the spotify api redirects during the auth process with the code value"""

    handler = generate_spotify_handler(request.args.get("code"))
    handler.pause_playback()

    return redirect("http://localhost:3000")

# Endpoint for the front end to access the necessary environment vars
@app.route("/env", methods=["GET"])
def get_environment():
    """Will send the client_id, scope, and redirect_uri to the frontend in json format"""
    
    env_vars = dotenv_values(".env")
    return {
        "client_id" : env_vars["CLIENT_ID"],
        "redirect_uri" : env_vars["REDIRECT_URI"],
        "scope" : env_vars["SCOPE"]
    }

# Getting the current User
@app.route("/user", methods=["GET"])
def get_user():
    """This function is used for accessing the cache and seeing if there is a currently logged in user"""

    env_vars = dotenv_values(".env")

    username = ""
    valid = False

    try:
        handler = load_spotify_handler(env_vars["CACHE_FILE"])
        username = handler.get_username()
        valid = True
    except Exception as e:
        # Nothing here on purpose
        print("User not cached")
        print(e)

    return {
        "valid": valid,
        "username": username,
    }


# Setting the current User


# Setting the Wifi Credentials
