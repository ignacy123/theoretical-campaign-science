#!/usr/bin/env python3
from flask import Flask, g, redirect, url_for
from users import blueprint as users_blueprint
from users import in_user_context, login, logout

app = Flask(__name__)
app.config['SECRET_KEY'] = 'verysecretkey'
app.register_blueprint(users_blueprint)

@app.route("/")
def main_page():
    return """
    <html>
    <body>
    <h1>Hello world</h1>
    </body>
    </html>
    """

@app.route("/login/<username>")
def login_page(username):
    login(username)
    return redirect(url_for('hello_user_page'))

@app.route("/logout")
def logout_page():
    logout()
    return redirect(url_for('main_page'))

@app.route("/hello_user")
@in_user_context
def hello_user_page():
    return f"""
    <html>
    <body>
    <h1>Hello {g.user}</h1>
    </body>
    </html>
    """