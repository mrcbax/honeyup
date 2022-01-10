#!/bin/bash

CONFIG_LOC=./res/config.env.txt

# Remove config.env.txt if it exists
rm -f $CONFIG_LOC

# Make new config.env.txt with docker-compose env variables
echo "[app]
id = ${APP_ID}

[aws]
aws_access_key_id = ${AWS_ACCESS_KEY_ID}
aws_secret_access_key = ${AWS_SECRET_ACCESS_KEY}
output = json
region = us-east-2

[email]
smtp = ${STMP}
address = ${ADDRESS}
password = ${PASSWORD}
" > $CONFIG_LOC

# Run Honeyup
./honeyup