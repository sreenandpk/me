// scratch/test-latency.js
"use strict";

const path = require("path");
const fs = require("fs");

const envPath = path.join(process.cwd(), ".env");
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

const handler = require("../api/chat.js");

const questions = [
  "What projects has Sreenand built?",
  "What technologies does he use?",
  "Tell me about CareStream.",
  "How can I contact Sreenand?",
];

async function run() {
  for (const q of questions) {
    console.log(`\n========================================`);
    console.log(`Testing Question: "${q}"`);
    
    let resultData = null;
    let statusCode = 200;

    const req = {
      method: "POST",
      body: { question: q }
    };

    const res = {
      setHeader: () => {},
      status: (code) => { statusCode = code; return res; },
      json: (data) => { resultData = data; return res; },
      end: () => {}
    };

    const start = Date.now();
    await handler(req, res);
    const end = Date.now();

    console.log(`Status: ${statusCode}`);
    console.log(`Total duration: ${end - start} ms`);
    if (resultData?.answer) {
      console.log(`Answer length: ${resultData.answer.length} chars`);
      console.log(`Answer snippet: ${resultData.answer.substring(0, 150)}...`);
    } else {
      console.log(`Error:`, resultData);
    }

    // Wait 13 seconds between requests to satisfy 5 RPM free tier rate limit
    await new Promise((r) => setTimeout(r, 13000));
  }
}

run();
