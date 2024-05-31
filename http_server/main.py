#!/usr/bin/env python3
from flask import Flask

app = Flask(__name__)

@app.route("/")
def main_page():
    return """
    <html>
    <body>
    <h1>Hello world</h1>
    </body>
    </html>
    """
