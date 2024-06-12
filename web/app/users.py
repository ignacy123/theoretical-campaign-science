from flask import g, session, Blueprint, abort, current_app
from functools import wraps
import re

blueprint = Blueprint("users", __name__)

# Before each request
@blueprint.before_app_request
def prepare_user_data():
    g.user = None
    g.admin = ('admin' in session)
    if 'user' in session:
        g.user = session['user']


# Decorator meaning that the request is valid only with the user logged in
def in_user_context(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        if g.user is None:
            return abort(401)
        return func(*args, **kwargs)
    return wrapper

def set_session(username : str):
    session['user'] = username
    g.user = username

def stop_session():
    if 'user' in session:
        del(session['user'])
    g.user = None

def validate_login(login:str):
    if not (4 <= len(login) <= 64):
        raise ValueError

    pattern = '[A-Za-z0-9.~_-]*[A-Za-z0-9][A-Za-z0-9.~_-]*' # RFC 3986
    if re.fullmatch(pattern, login) is None:
        raise ValueError


def in_admin_context(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        if not g.admin:
            return abort(401)
        return func(*args, **kwargs)
    return wrapper

def set_admin_session():
    session['admin'] = True # value doesn't matter
    g.admin = True

def is_in_admin_session():
    return g.admin

def stop_admin_session():
    if 'admin' in session:
        del(session['admin'])
    g.admin = False

def admin_password_correct(password:str):
    return password == current_app.config['ADMIN_PASSWORD']