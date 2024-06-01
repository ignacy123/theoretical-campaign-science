#!/usr/bin/env python3
import flask
from users import blueprint as users_blueprint
from users import in_user_context, login, logout

app = flask.Flask(__name__)
app.config['SECRET_KEY'] = 'verysecretkey'
app.register_blueprint(users_blueprint)

@app.route("/")
def main_page():
    return flask.render_template('main.html')

@app.route("/login/<username>")
def login_page(username):
    login(username)
    return flask.redirect(flask.url_for('hello_user_page'))

@app.route("/logout")
def logout_page():
    logout()
    return flask.redirect(flask.url_for('main_page'))

@app.route("/hello_user")
@in_user_context
def hello_user_page():
    return flask.render_template('hello_user.html')