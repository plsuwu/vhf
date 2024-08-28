#!/bin/bash

set -uxo pipefail

# pkill nginx
nginx -c /app/nginx.conf
nginx -s reload

go run server.go
