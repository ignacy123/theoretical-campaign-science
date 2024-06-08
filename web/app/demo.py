import flask
from .database import with_db_session
from .users import  in_user_context, login, logout

blueprint = flask.Blueprint("test", __name__)

@blueprint.route("/")
def main_page():
    return flask.render_template('demo/main.html')

@blueprint.route("/login/<username>")
def login_page(username):
    login(username)
    return flask.redirect(flask.url_for('.hello_user_page'))

@blueprint.route("/logout")
def logout_page():
    logout()
    return flask.redirect(flask.url_for('.main_page'))

@blueprint.route("/hello_user")
@in_user_context
def hello_user_page():
    return flask.render_template('demo/hello_user.html')

@blueprint.route("/db")
@with_db_session
def db_test_page(db):
    db.execute('SELECT * from test')
    data = {row.key:row.value for row in db.fetchall()}
    return flask.render_template("demo/db_test.html", data=data)