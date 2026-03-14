"use client";

import { useEffect, useState } from "react";

interface ApiHealth {
  status: string;
  version: string;
  db_connected: boolean;
}

export default function Home() {
  const [health, setHealth] = useState<ApiHealth | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const apiUrl = process.env.API_URL || "http://localhost:8080";
    fetch(`${apiUrl}/health`)
      .then((res) => res.json())
      .then((data: ApiHealth) => setHealth(data))
      .catch((err) => setError(err.message));
  }, []);

  return (
    <main style={{ padding: "2rem", fontFamily: "system-ui, sans-serif" }}>
      <h1>SDD Navigator</h1>
      <p>Frontend is running.</p>

      <h2>API Health</h2>
      {error && <p style={{ color: "red" }}>Error: {error}</p>}
      {health ? (
        <ul>
          <li>Status: {health.status}</li>
          <li>Version: {health.version}</li>
          <li>DB Connected: {health.db_connected ? "Yes" : "No"}</li>
        </ul>
      ) : (
        !error && <p>Loading...</p>
      )}
    </main>
  );
}
