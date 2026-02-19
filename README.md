terminal 1

source venv/bin/activate 
cd ~/simplec2/server 
uvicorn main:app --host 0.0.0.0 --port 8000 --reload

terminal 2

cd ~/simplec2/agent        
cargo build --release

./target/release/rust-agent


terminal 3
# Replace <AGENT_ID_HERE> with the real UUID you saw
curl -X POST http://127.0.0.1:8000/task/<AGENT_ID_HERE> \
  -H "Content-Type: application/json" \
  -d '{"id":"task-hello-1", "command":"whoami"}'

#output
{"status":"queued"}

