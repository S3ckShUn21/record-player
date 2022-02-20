import { useBackendAlive } from "./components/BackendApi";
import { CredentialLogin } from "./components/CredentialLogin";

const App = () => {
  const {alive: backendAlive, loading} = useBackendAlive();

  return (
    <center>
      <h2><u>Record Player UI</u></h2>
      <p>Backend Status: { loading ? '⏳': ( backendAlive ? '✅' : '🆘') }</p>
      { backendAlive &&
        <CredentialLogin />
      }

    </center>
  );
};

export default App;
