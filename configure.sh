#!/bin/sh

#db init
echo "'run first 'create database voting;'"

#scripts docker infrastructor
chomod +x scripts/docker_compose.sh

#rust infrastructor
rustup override set nightly