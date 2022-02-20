import axios from "axios";
import { useEffect, useState } from "react";
import querystring from "querystring";

function craftRandomState() {
  return Math.random().toString(16).substring(2);
}

function craftLoginRequest(setLoginURL) {
  let credentials = null;

  // Get credentials from the backend
  axios
    .get("http://localhost:5000/env")
    .then((res) => {
      credentials = res.data;
      setLoginURL(
        "https://accounts.spotify.com/authorize?" +
          querystring.stringify({
            client_id: credentials.client_id,
            response_type: "code",
            redirect_uri: credentials.redirect_uri,
            state: craftRandomState(),
            scope: credentials.scope,
            show_dialog: "true",
          })
      );
    })
    .catch((e) => {
      console.log(e);
      return null;
    });
}

export const CredentialLogin = () => {
  const [loginURL, setLoginURL] = useState("");
  const [userData, setUserData] = useState({ logged_in: false, user_name: "" });

  useEffect(() => {
    craftLoginRequest(setLoginURL);
  }, []);

  useEffect(() => {
    axios
      .get("http://localhost:5000/user")
      .then((res) => {
        setUserData(res.data);
      })
      .catch((err) => {
        console.log(err);
      });
  }, []);

  return (userData.user_name === "") 
  ? (<button onClick={() => {window.open(loginURL, "_self");}}>Log In</button>)
  : (<p>Logged in as: {userData.user_name}</p>);
};
