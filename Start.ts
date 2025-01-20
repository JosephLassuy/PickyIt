#!/usr/bin/env bun

import { $ } from "bun";

async function main() {
  // Check if docker compose is already running
  try {
    const running = await $`docker compose ps --status running | grep crdb`.text();
    if (running) {
      console.log("Docker Compose is already running!");
      process.exit(0);
    }
  } catch {
    // grep returns non-zero exit code if no matches found, which is fine
  }

  // Start Docker Compose in detached mode
  console.log("Starting Docker Compose...");
  await $`docker compose up -d`;

  // Wait for database to be ready (max 30 seconds)
  console.log("Waiting for database to be ready...");
  for (let i = 0; i < 30; i++) {
    try {
      await $`docker compose exec crdb cockroach sql --insecure --host=localhost --execute="SELECT 1"`.quiet();
      console.log("Database is ready!");
      process.exit(0);
    } catch {
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
  }

  console.error("Error: Database failed to start within 30 seconds");
  process.exit(1);
}

main().catch(err => {
  console.error(err);
  process.exit(1);
}); 