#/bin/bash

echo "Entering Dev Env"
docker run --rm -it -v "$(pwd):/app" pusha_dev
