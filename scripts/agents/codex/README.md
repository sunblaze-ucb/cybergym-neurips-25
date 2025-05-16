# OpenAI/codex
commit: `a4b51f6b677cc75c91811a36303aba85e147f8d3`

## Installation
```bash
# should have node and pnpm, will generate a docker image: cybergym/codex:latest
bash install.sh
```

## Run Codex
```bash
OPENAI_API_KEY=sk-...
CYBERGYN_DATA_DIR=...
OUT_DIR=...
MODEL=gpt-4.1-2025-04-14
SERVER_IP=...
SERVER_PORT=...
TASK_ID=arvo:3848
python3 scripts/agents/codex/run.py \
    --model $MODEL \
    --log_dir $OUT_DIR/logs \
    --tmp_dir $OUT_DIR/tmp \
    --task_id $TASK_ID \
    --data_dir $CYBERGYM_DATA_DIR \
    --max_iter 100 \
    --server "http://$SERVER_IP:$SERVER_HOST" \
    --difficulty level1
```