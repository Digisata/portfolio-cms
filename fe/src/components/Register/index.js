import React, { useState } from "react";
import Swal from "sweetalert2";
import { register } from "../../utils/api"; // adjust the path based on your folder structure

const Register = ({ setIsRegister }) => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [retypePassword, setRetypePassword] = useState("");

  const validatePasswords = () => {
    if (!password || !retypePassword) {
      Swal.fire({
        icon: "error",
        title: "Error!",
        text: "Please fill in both password fields",
        showConfirmButton: true,
      });
      return false;
    }

    if (password !== retypePassword) {
      Swal.fire({
        icon: "error",
        title: "Error!",
        text: "Passwords do not match",
        showConfirmButton: true,
      });
      return false;
    }

    if (password.length < 6) {
      Swal.fire({
        icon: "error",
        title: "Error!",
        text: "Password must be at least 6 characters long",
        showConfirmButton: true,
      });
      return false;
    }

    return true;
  };

  const handleRegister = async (e) => {
    e.preventDefault();

    // Validate passwords before making API call
    if (!validatePasswords()) {
      return;
    }

    try {
      await register({ email, password });
      localStorage.setItem("is_authenticated", "false");
      setIsRegister(false);
      Swal.fire({
        icon: "success",
        title: "Successfully registered!",
        showConfirmButton: false,
        timer: 1500,
      });
      // Clear form after successful registration
      setEmail("");
      setPassword("");
      setRetypePassword("");
    } catch (error) {
      Swal.fire({
        icon: "error",
        title: "Error!",
        text: error.message || error,
        showConfirmButton: true,
      });
    }
  };

  return (
    <div className="small-container">
      <form onSubmit={handleRegister}>
        <h1>Admin Register</h1>
        <label htmlFor="email">Email</label>
        <input
          id="email"
          type="email"
          name="email"
          placeholder="admin@example.com"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          required
        />
        <label htmlFor="password">Password</label>
        <input
          id="password"
          type="password"
          name="password"
          placeholder="Enter password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
          minLength="6"
        />
        <label htmlFor="retypePassword">Retype Password</label>
        <input
          id="retypePassword"
          type="password"
          name="retypePassword"
          placeholder="Confirm password"
          value={retypePassword}
          onChange={(e) => setRetypePassword(e.target.value)}
          required
          minLength="6"
        />
        <div style={{ marginTop: "30px" }}>
          <input type="submit" value="Register" />
          <input
            type="button"
            className="muted-button"
            value="Cancel"
            style={{ marginLeft: "12px" }}
            onClick={() => setIsRegister(false)}
          />
        </div>
      </form>
    </div>
  );
};

export default Register;
