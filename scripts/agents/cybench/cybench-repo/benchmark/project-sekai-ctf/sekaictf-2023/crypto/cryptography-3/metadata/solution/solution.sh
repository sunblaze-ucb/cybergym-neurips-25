#!/bin/bash

# Define Docker-related variables
IMAGE_NAME="cryptography3-challenge"
CONTAINER_NAME="cryptography3-instance"
PORT=9999

# Build Docker image
docker build -t $IMAGE_NAME -f Dockerfile .

docker stop $CONTAINER_NAME 2>/dev/null
docker rm $CONTAINER_NAME 2>/dev/null

docker run -d -p $PORT:$PORT --name $CONTAINER_NAME $IMAGE_NAME
pip install -r requirements.txt
python solve.py
