from dataclasses import dataclass, field
from dataclasses_json import dataclass_json, config
from datetime import datetime, timedelta, timezone
import requests
from requests import Response
from dotenv import dotenv_values
import base64


@dataclass_json
@dataclass
class SpotifyHandler:
    """Used to handle all spotify api requests"""

    access_token: str
    refresh_token: str
    cache_file: str
    expiration: datetime

    # Private Helper Functions

    def _cache(self):
        """Saves the current data to the set cache_file location"""

        with open(self.cache_file, "w") as file:
            file.write(self.to_json())

    def _check_refresh(self):
        """Will check if the tokens are expired and update them if need be"""

        if datetime.now(timezone.utc) > self.expiration:
            env_vars = dotenv_values(".env")
            res = requests.post(
                "https://accounts.spotify.com/api/token",
                data={"grant_type": "refresh_token", "refresh_token": self.refresh_token},
                headers={
                    "Authorization": _encode_basic_auth(env_vars["CLIENT_ID"], env_vars["CLIENT_SECRET"]),
                    "Content-Type": "application/x-www-form-urlencoded",
                },
            )

            # refresh the access token and the expiration time
            response_json = res.json()
            self.expiration = datetime.now(timezone.utc) + timedelta(seconds=response_json["expires_in"])
            self.access_token = response_json["access_token"]
            # save the updated data
            self._cache()

    # Public API functions

    def get_username(self) -> str:
        """Queries the API for /me endpoint and extracts the username"""
        self._check_refresh()
        res = requests.get(
            "https://api.spotify.com/v1/me",
            headers={"Authorization": "Bearer " + self.access_token, "Content-Type": "application/json"},
        )
        _handle_api_response_error(res)
        return res.json().get("display_name")

    def pause_playback(self):
        """Will pause playback of the user"""
        self._check_refresh()
        res = requests.post(
            "https://api.spotify.com/v1/me/player/pause",
            headers={"Authorization": "Bearer " + self.access_token, "Content-Type": "application/json"},
        )
        _handle_api_response_error(res)


#
#
#                           Helper Functions
#
#


def _handle_api_response_error(response: Response):
    """Looks at the response and will print ot the error if there is a bad status"""
    if response.status_code != 200:
        print(response.text)


def _encode_basic_auth(client_id: str, client_secret: str) -> str:
    byte_string = bytes(client_id + ":" + client_secret, encoding="utf-8")
    return "Basic " + str(base64.b64encode(byte_string), encoding="utf-8")


def load_spotify_handler(filepath: str) -> SpotifyHandler:
    """Will generate a SpotifyHandler object from a given json file"""

    with open(filepath, "r") as cache_file:
        raw_json = cache_file.read()

    return SpotifyHandler.from_json(raw_json)


def generate_spotify_handler(code: str, cache_file: str = "user.json") -> SpotifyHandler:
    """Will generate a SpotifyHandler object using the spotify api and a code"""

    # TODO : Might want to refactor this some how, how it does't do this every func call
    env_vars = dotenv_values(".env")

    res = requests.post(
        "https://accounts.spotify.com/api/token",
        data={"grant_type": "authorization_code", "code": code, "redirect_uri": env_vars["REDIRECT_URI"]},
        headers={
            "Authorization": _encode_basic_auth(env_vars["CLIENT_ID"], env_vars["CLIENT_SECRET"]),
            "Content-Type": "application/x-www-form-urlencoded",
        },
    )

    response_json = res.json()

    handler = SpotifyHandler(
        access_token=response_json.get("access_token"),
        refresh_token=response_json.get("refresh_token"),
        expiration=datetime.now(timezone.utc) + timedelta(seconds=response_json.get("expires_in")),
        cache_file=cache_file,
    )

    # Make sure to save this new handler we just created
    handler._cache()

    return handler
