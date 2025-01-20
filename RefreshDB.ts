#!/usr/bin/env bun

async function isDockerRunning() {
  try {
    const proc = Bun.spawn(['docker', 'compose', 'ps', '--status', 'running']);
    const output = await new Response(proc.stdout).text();
    return output.includes('crdb');
  } catch (error) {
    return false;
  }
}

async function waitForDatabase() {
  for (let i = 0; i < 30; i++) {
    try {
      const proc = Bun.spawn([
        'docker', 'compose', 'exec', 'crdb',
        'cockroach', 'sql', '--insecure', '--host=localhost',
        '--execute=SELECT 1'
      ], {
        stderr: 'ignore',
        stdout: 'ignore'
      });
      
      const exitCode = await proc.exited;
      if (exitCode === 0) return true;
    } catch {}
    
    console.log("Waiting for database to be ready...");
    await Bun.sleep(1000);
  }
  return false;
}

async function main() {
  if (!await isDockerRunning()) {
    console.log("Starting Docker Compose...");
    const startProc = Bun.spawn(['docker', 'compose', 'up', '-d']);
    await startProc.exited;
    
    if (!await waitForDatabase()) {
      console.error("Database failed to start within 30 seconds");
      process.exit(1);
    }
  }

  console.log("Applying down migrations...");
  const downProc = Bun.spawn([
    'docker', 'compose', 'exec', 'crdb',
    'cockroach', 'sql', '--insecure', '--host=localhost',
    '--file=/migration/down.sql'
  ]);
  await downProc.exited;

  console.log("Applying up migrations...");
  const upProc = Bun.spawn([
    'docker', 'compose', 'exec', 'crdb',
    'cockroach', 'sql', '--insecure', '--host=localhost',
    '--file=/migration/up.sql'
  ]);
  await upProc.exited;

  console.log("Database refresh complete!");
}

main().catch(error => {
  console.error("Error:", error);
  process.exit(1);
});
