// scratch/phase3-benchmark.js
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

const testQueries = [
  "What projects has Sreenand built?",
  "What technologies does Sreenand use?",
  "Tell me about CareStream.",
  "How does CareStream use Redis?",
  "What is Just Listen?",
  "How does the trading platform use PostgreSQL?",
  "How can I contact Sreenand?",
  "What is Sreenand's engineering approach?",
  "What is Sreenand's favorite movie?",
  "What is Sreenand currently learning?",
];

async function runBenchmark() {
  console.log("=== PHASE 3 PRODUCTION VERIFICATION BENCHMARK ===\n");
  const results = [];

  for (let i = 0; i < testQueries.length; i++) {
    const q = testQueries[i];
    console.log(`[${i + 1}/${testQueries.length}] Query: "${q}"`);

    let statusCode = 200;
    let resultData = null;
    let timingLog = null;

    // Capture console.log from api/chat.js
    const originalLog = console.log;
    console.log = (...args) => {
      const msg = args.join(" ");
      if (msg.includes("[chat.js] Timings")) {
        timingLog = msg;
      }
      originalLog(...args);
    };

    const req = {
      method: "POST",
      body: { question: q },
    };

    const res = {
      setHeader: () => {},
      status: (code) => {
        statusCode = code;
        return res;
      },
      json: (data) => {
        resultData = data;
        return res;
      },
      end: () => {},
    };

    const tStart = Date.now();
    await handler(req, res);
    const tEnd = Date.now();

    console.log = originalLog; // restore log

    // Parse timingLog
    // Format: [chat.js] Timings -> retrieval_ms: 1ms | gemini_ms: 2500ms | total_ms: 2501ms | sections: 7 | tokens: ~793
    let retrieval_ms = 0;
    let gemini_ms = tEnd - tStart;
    let total_ms = tEnd - tStart;
    let sections = 0;
    let tokens = 0;

    if (timingLog) {
      const rMatch = timingLog.match(/retrieval_ms:\s*(\d+)ms/);
      const gMatch = timingLog.match(/gemini_ms:\s*(\d+)ms/);
      const totMatch = timingLog.match(/total_ms:\s*(\d+)ms/);
      const sMatch = timingLog.match(/sections:\s*(\d+)/);
      const tokMatch = timingLog.match(/tokens:\s*~(\d+)/);

      if (rMatch) retrieval_ms = parseInt(rMatch[1], 10);
      if (gMatch) gemini_ms = parseInt(gMatch[1], 10);
      if (totMatch) total_ms = parseInt(totMatch[1], 10);
      if (sMatch) sections = parseInt(sMatch[1], 10);
      if (tokMatch) tokens = parseInt(tokMatch[1], 10);
    }

    const context_chars = tokens * 4;

    const record = {
      index: i + 1,
      query: q,
      retrieved_sections: sections,
      context_chars: context_chars,
      context_token_estimate: tokens,
      retrieval_ms: retrieval_ms,
      gemini_ms: gemini_ms,
      total_ms: total_ms,
      http_status: statusCode,
      answer_length: resultData?.answer ? resultData.answer.length : 0,
      answer_snippet: resultData?.answer ? resultData.answer.substring(0, 100) + "..." : (resultData?.error || "N/A"),
    };

    results.push(record);

    console.log(`    Status: ${statusCode} | retrieval_ms: ${retrieval_ms}ms | gemini_ms: ${gemini_ms}ms | total_ms: ${total_ms}ms`);
    console.log(`    Context: ${sections} sections (~${tokens} tokens / ${context_chars} chars)`);
    console.log(`    Answer Snippet: ${record.answer_snippet}\n`);

    // Wait 13 seconds between queries to avoid hitting Gemini Free Tier 5 RPM rate limit
    if (i < testQueries.length - 1) {
      console.log(`    [Waiting 13s for Gemini free tier rate limit window...]`);
      await new Promise((r) => setTimeout(r, 13000));
    }
  }

  console.log("\n========================================================");
  console.log("FINAL BENCHMARK RESULTS SUMMARY:");
  console.log("========================================================");
  console.table(
    results.map((r) => ({
      Query: r.query.substring(0, 30),
      Sections: r.retrieved_sections,
      "Tokens (~": r.context_token_estimate,
      Ret_ms: r.retrieval_ms,
      Gemini_ms: r.gemini_ms,
      Total_ms: r.total_ms,
      Status: r.http_status,
    }))
  );

  const totalGeminiMs = results.reduce((acc, r) => acc + r.gemini_ms, 0);
  const avgGeminiMs = Math.round(totalGeminiMs / results.length);
  const totalMs = results.reduce((acc, r) => acc + r.total_ms, 0);
  const avgTotalMs = Math.round(totalMs / results.length);

  console.log(`\nAverage gemini_ms: ${avgGeminiMs} ms`);
  console.log(`Average total_ms: ${avgTotalMs} ms`);
}

runBenchmark();
