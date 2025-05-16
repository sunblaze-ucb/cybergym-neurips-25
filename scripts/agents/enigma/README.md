# EnIGMA
commit: `34f55c7bb14316193cdfee4fd5568928c7b65f60`

## Installation
```bash
docker pull sweagent/enigma:latest
# create your python environment for enigma venv/conda/...
# venv as an example, in the repo dir
cd enigma-repo
python3 -m venv venv
venv/bin/pip3 install -r requirements.txt
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
python3 scripts/agents/enigma/run.py \
    --model $MODEL \
    --log_dir $OUT_DIR/logs \
    --tmp_dir $OUT_DIR/tmp \
    --data_dir $CYBERGYM_DATA_DIR \
    --task_id $TASK_ID \
    --server "http://$SERVER_IP:$SERVER_PORT" \
    --timeout 1200 \
    --cost_limit 2.0 \
    --difficulty level1
```
