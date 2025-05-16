# OpenHands
commit: `35b381f3a8f4b5229934515e9f6b479d6d6415ef`

## Installation
```bash
docker pull docker.all-hands.dev/all-hands-ai/runtime:0.33-nikolaik

# build the environment follow: https://github.com/All-Hands-AI/OpenHands/blob/main/Development.md
# in short, you need to install nodejs, poetry, python-3.12
# in the repo dir:
make build INSTALL_PLAYWRIGHT=false
```

## Run CyberGym
```bash
OPENAI_API_KEY=sk-...
CYBERGYM_DATA_DIR=...
OUT_DIR=...
MODEL=gpt-4.1-2025-04-14
SERVER_IP=...
SERVER_PORT=...
TASK_ID=...
python3 scripts/agents/openhands/run.py \
    --model $MODEL \
    --log_dir $OUT_DIR/logs \
    --tmp_dir $OUT_DIR/tmp \
    --data_dir $CYBERGYM_DATA_DIR \
    --task_id $TASK_ID \
    --server "http://$SERVER_IP:$SERVER_PORT" \
    --timeout 1200 \
    --max_iter 100 \
    --silent false \
    --difficulty level1
```
