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
1. Full data
```bash
CYBERGYM_DATA_DIR=./cybergym_data
python scripts/server_data/download.py --tasks-file $CYBERGYM_DATA_DIR/tasks.json
wget https://huggingface.co/datasets/sunblaze-ucb/cybergym-server/resolve/main/cybergym-oss-fuzz-data.7z
7z x cybergym-oss-fuzz-data.7z
```

2. Subset data
The full server data is large (~10TB). We provide a subset with the following 10 tasks, which include 5 tasks that the agent can successfully generate the PoC and 5 tasks that are not easy for the agent.
```
arvo:47101
arvo:3938
arvo:24993
arvo:1065
arvo:10400
arvo:368
oss-fuzz:42535201
oss-fuzz:42535468
oss-fuzz:370689421
oss-fuzz:385167047
```
Download the subset data
```bash
python scripts/server_data/download_subset.py
wget https://huggingface.co/datasets/sunblaze-ucb/cybergym-server/resolve/main/cybergym-oss-fuzz-data-subset.7z
7z x cybergym-oss-fuzz-data-subset.7z
```

Start the PoC submission server:
```bash
PORT=... # port of the server
POC_SAVE_DIR=... # dir to save the pocs
CYBERGYM_SERVER_DATA_DIR=./oss-fuzz-data
python3 -m cybergym.server \
    --host 0.0.0.0 --port $PORT \
    --log_dir $POC_SAVE_DIR --db_path $POC_SAVE_DIR/poc.db \
    --cybergym_oss_fuzz_path $CYBERGYM_SERVER_DATA_DIR
```

Test:
```bash
# generate the task
TASK_ID='arvo:10400'
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
