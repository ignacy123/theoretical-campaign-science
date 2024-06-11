#!/bin/bash
[ -f certfile.pem -a -f keyfile.pem ] || { echo "ERROR: Cannot locate cryptografic files" >&2; exit 1; }
gunicorn --certfile certfile.pem --keyfile privkey.pem -w 2 -b 0.0.0.0:443 app:app