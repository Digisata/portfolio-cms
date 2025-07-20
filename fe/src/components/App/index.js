import React, { useState, useEffect } from "react";

import Login from "../Login";
import Register from "../Register";
import Dashboard from "../Dashboard";

const App = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(null);
  const [isRegister, setIsRegister] = useState(false);

  useEffect(() => {
    setIsAuthenticated(JSON.parse(localStorage.getItem("is_authenticated")));
  }, []);

  return (
    <>
      {isAuthenticated ? (
        <Dashboard setIsAuthenticated={setIsAuthenticated} />
      ) : isRegister ? (
        <Register setIsRegister={setIsRegister} />
      ) : (
        <Login setIsAuthenticated={setIsAuthenticated} setIsRegister={setIsRegister} />
      )}
    </>
  );
};

export default App;
