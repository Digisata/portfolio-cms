import React, { useState } from "react";
import Swal from "sweetalert2";
import { login } from "../../utils/api"; // adjust the path based on your folder structure

const Login = ({ setIsAuthenticated, setIsRegister }) => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  const handleLogin = async (e) => {
    e.preventDefault();

    try {
      const data = await login({ email, password });

      localStorage.setItem("is_authenticated", "true");
      localStorage.setItem("token", data.jwt);
      setIsAuthenticated(true);

      Swal.fire({
        icon: "success",
        title: "Successfully logged in!",
        showConfirmButton: false,
        timer: 1500,
      });
    } catch (error) {
      Swal.fire({
        icon: "error",
        title: "Error!",
        text: "Incorrect email or password.",
        showConfirmButton: true,
      });
    }
  };

  return (
    <div className="small-container">
      <form onSubmit={handleLogin}>
        <h1>Admin Login</h1>
        <label htmlFor="email">Email</label>
        <input
          id="email"
          type="email"
          name="email"
          placeholder="admin@example.com"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
        <label htmlFor="password">Password</label>
        <input
          id="password"
          type="password"
          name="password"
          placeholder="qwerty"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <div style={{ marginTop: "30px" }}>
          <input type="submit" value="Login" />
          <input
            type="button"
            className="muted-button"
            value="Register"
            style={{ marginLeft: "12px" }}
            onClick={() => setIsRegister(true)}
          />
        </div>
      </form>
    </div>
  );
};

export default Login;
