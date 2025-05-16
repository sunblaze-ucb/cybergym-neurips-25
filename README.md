# CyberGym: A Evaluating AI Agents’ Cybersecurity Capabilities with Real-World Vulnerabilities at Scale

## Installation
Require python and docker environment.

Install the dependencies for the server and the task generation
```bash
pip3 install -e '.[dev,server]'
```

Download the benchmark data
```bash
git lfs install
git clone https://huggingface.co/datasets/sunblaze-ucb/cybergym cybergym_data
```

### Agents
For different agents, please follow the instructions in the separate folders
- [Cybench](scripts/agents/cybench/README.md)
- [ENiGMA](scripts/agents/enigma/README.md)
- [Codex](scripts/agents/codex/README.md)
- [OpenHands](scripts/agents/openhands/README.md)

## Evaluation
Download the PoC submission server data
```bash

```

Start the PoC submission server:
```bash
PORT=... # port of the server
POC_SAVE_DIR=... # dir to save the pocs
python3 -m cybergym.server \
    --host 0.0.0.0 --port $PORT \
    --log_dir $POC_SAVE_DIR --db_path $POC_SAVE_DIR/poc.db \
    --cybergym_oss_fuzz_path $CYBERGYM_DATA_DIR
```

Test:
```bash
# generate the task
TASK_ID='arvo:3848'
OUT_DIR=PATH_TO_A_TEMP_DIR
CYBERGYM_DATA_DIR=...
python3 -m cybergym.task.gen_task \
    --task-id $TASK_ID \
    --out-dir $OUT_DIR \
    --data-dir $CYBERGYM_DATA_DIR \
    --server "http://$SERVER_IP:$SERVER_PORT" \
    --difficulty level1

# example output
# ├── description.txt
# ├── README.md
# ├── repo-vul.tar.gz
# └── submit.sh

# try the submission
echo -en "\x00\x01\x02\x03" > $OUT_DIR/poc
bash $OUT_DIR/submis.sh $OUT_DIR/poc

# example return
# {"task_id":"arvo:3848","exit_code":0,"output":"INFO: Seed: 779112339\nINFO: Loaded 1 modules   (6096 guards): 6096 [0x965580, 0x96b4c0), \n/out/pe_fuzzer: Running 1 inputs 1 time(s) each.\nRunning: /tmp/poc\nExecuted /tmp/poc in 3 ms\n***\n*** NOTE: fuzzing was not performed, you have only\n***       executed the target code on a fixed set of inputs.\n***\n","poc_id":"8f20a76a34d0482a82da247f96b39f01"}
```
