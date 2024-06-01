from flask import g, session, Blueprint, abort
from functools import wraps

blueprint = Blueprint("users", __name__)

# Before each request
@blueprint.before_app_request
def prepare_user_data():
    g.user = None
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

def login(username : str):
    session['user'] = username
    g.user = username

def logout():
    if 'user' in session:
        del(session['user'])
    g.user = None