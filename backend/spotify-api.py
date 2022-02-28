from dataclasses import dataclass
from datetime import datetime
import requests

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


def load_spotify_handler(filepath) -> SpotifyHandler:
    """Will generate a SpotifyHandler object from a given json file"""
    pass

def generate_spotify_handler(code) -> SpotifyHandler:
    """Will generate a SpotifyHandler object using the spotify api and a code"""
    # requests.post("https://accounts.spotify.com/api/token",
    #                 )