#!/bin/bash
[ -f certfile.pem -a -f keyfile.pem ] || exit "Cannot locate cryptografic files"
gunicorn --certfile certfile.pem --keyfile privkey.pem -w 2 -b 0.0.0.0:443 app:app