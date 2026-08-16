// scripts/dev-api.js
// Local standalone HTTP server for testing /api/chat alongside `dx serve`.
// Usage: node scripts/dev-api.js

"use strict";

const http = require("http");
const path = require("path");
const fs = require("fs");
const handler = require("../api/chat.js");

// ── Load .env ─────────────────────────────────────────────────────────────
const envPath = path.join(__dirname, "..", ".env");
if (fs.existsSync(envPath)) {
  const envContent = fs.readFileSync(envPath, "utf-8");
  for (const line of envContent.split("\n")) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;
    const eqIdx = trimmed.indexOf("=");
    if (eqIdx === -1) continue;
    const key = trimmed.slice(0, eqIdx).trim();
    const value = trimmed.slice(eqIdx + 1).trim();
    if (!process.env[key]) process.env[key] = value;
  }
}

const PORT = 3001;

const server = http.createServer((req, res) => {
  let body = "";
  req.on("data", (chunk) => {
    body += chunk;
  });
  req.on("end", () => {
    try {
      req.body = body ? JSON.parse(body) : {};
    } catch (e) {
      req.body = {};
    }

    // Express-like compatibility helpers for api/chat.js
    res.status = function (code) {
      res.statusCode = code;
      return res;
    };
    res.json = function (data) {
      res.setHeader("Content-Type", "application/json");
      res.end(JSON.stringify(data));
      return res;
    };

    handler(req, res).catch((err) => {
      console.error("[dev-api] Handler error:", err);
      res.status(500).json({ error: "Internal server error." });
    });
  });
});

server.listen(PORT, () => {
  console.log(`[dev-api] Local RAG API server running at http://localhost:${PORT}/api/chat`);
});
