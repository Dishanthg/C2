terminal 1

cd ~/simplec2/server   # or wherever your main.py actually is
# If venv not active:
# source venv/bin/activate   (or global if you installed without venv)
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

