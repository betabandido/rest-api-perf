#!/bin/bash

go get fortio.org/fortio

GOPATH=$(go env GOPATH)
echo "PATH=\$PATH:$GOPATH/bin" >> ~/.profile
source ~/.profile
