import { useEffect, useState } from "react";

export const useBackendAlive = () => {
  const [state, setState] = useState({ alive: false, loading: true });

  useEffect(() => {
    fetch('http://localhost:5000/alive')
      .catch((err) => {
        console.log(err);
        setState({
          alive: false,
          loading: false,
        });
      })
      .then((res) => {
        setState({ alive: (res && res.status < 500), loading: false });
      });
  }, []);

  return state;
};
