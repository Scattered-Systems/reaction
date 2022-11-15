#!/usr/bin/env bash
npm install -g npm@9.1.1
npm install
npm run build
sudo apt -y update && sudo apt -y upgrade && sudo apt -y autoremove
sudo apt -y install apt-utils protobuf-compiler