import { useState } from "react";

const Login: React.FC = function () {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  function onSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    fetch("/api/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username,
        password,
      }),
    });
  }

  return (
    <form onSubmit={onSubmit}>
      <label>
        Username:
        <input
          name="username"
          type="text"
          required
          onChange={(e) => setUsername(e.target.value)}
        />
      </label>
      <label>
        Password:
        <input
          name="password"
          type="password"
          required
          onChange={(e) => setPassword(e.target.value)}
        />
      </label>
      <input type="submit" value="Log in" />
    </form>
  );
};

export default Login;
