#!/usr/bin/env bun

import { readFileSync } from 'fs';
import { Client } from 'pg';

async function main() {
  const connectionString = process.env.DATABASE_URL;
  if (!connectionString) {
    throw new Error('DATABASE_URL environment variable is not set');
  }

  try {
    // Read the SQL file
    const sql = readFileSync('migration/up.sql', 'utf-8');

    // Create a Postgres client
    const client = new Client({ connectionString });
    await client.connect();

    console.log('Running migration...');
    await client.query(sql);
    
    console.log('Migration completed successfully!');
    await client.end();
  } catch (error) {
    console.error('Migration failed:', error);
    process.exit(1);
  }
}

main(); 