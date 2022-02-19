import React, { useState, useEffect } from "react";
import { useBackendAlive } from "./components/BackendApi";
import { CredentialLogin } from "./components/CredentialLogin";

const App = () => {
  const {alive: backendAlive, loading} = useBackendAlive();

  return (
    <div>
      <h1>Record Player UI</h1>
      <div>Backend Status: { loading ? 'Connecting...': ( backendAlive ? 'Up' : ' Down') }</div>
      { backendAlive &&
        <CredentialLogin />
      }

    </div>
  );
};

export default App;
