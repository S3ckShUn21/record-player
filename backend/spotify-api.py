from dataclasses import dataclass
from dataclasses_json import dataclass_json
from datetime import datetime, timedelta
import requests
from dotenv import dotenv_values
import base64

@dataclass_json
@dataclass
class SpotifyHandler:
    """Used to handle all spotify api requests"""

    access_token: str
    refresh_token : str
    expiration : datetime
    cache_file : str

    def check_refresh(self):
        """Will check if the tokens are expired and update them if need be"""
        pass

    def cache(self):
        """Saves the current data to the set cache_file location"""
        
        with open(self.cache_file, "w") as file:
            file.write( self.to_json() )


def load_spotify_handler(filepath : str) -> SpotifyHandler:
    """Will generate a SpotifyHandler object from a given json file"""
    
    with open(filepath, "r") as cache_file:
        raw_json = cache_file.read()
    
    return SpotifyHandler.from_json(raw_json)

def generate_spotify_handler(code : str, cache_file : str = "user.json" ) -> SpotifyHandler:
    """Will generate a SpotifyHandler object using the spotify api and a code"""

    # TODO : Might want to refactor this some how, how it does't do this every func call
    env_vars = dotenv_values(".env") 

    res = requests.post("https://accounts.spotify.com/api/token",
                    data={
                      "grant_type" : "authorization_code",
                      "code" : code,
                      "redirect_uri" : env_vars["redirect_uri"]
                    },
                    headers={
                        "Authorization" : encode_basic_auth(env_vars["client_id"], env_vars["client_secret"]),
                        "Content-Type" : "application/x-www-form-urlencoded"
                    })

    handler = SpotifyHandler(
        access_token =  res.json["access_token"],
        refresh_token = res.json["refresh_token"],
        expiration =    datetime.now() + timedelta(seconds=res.json["expires_in"]),
        cache_file =    cache_file
    )

    # Make sure to save this new handler we just created
    handler.cache()

    return handler

def encode_basic_auth(client_id : str, client_secret : str) -> str:
    byte_string = bytes(client_id + ":" + client_secret)
    return "Basic " + str(base64.b64encode(byte_string))