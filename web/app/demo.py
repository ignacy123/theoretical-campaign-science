import flask
from .database import with_db_session
from .users import  in_user_context, set_session, stop_session

blueprint = flask.Blueprint("demo", __name__)

@blueprint.route("/")
def main_page():
    return flask.render_template('demo/main.html')

@blueprint.route("/login/<username>")
def login_page(username):
    set_session(username)
    return flask.redirect(flask.url_for('.hello_user_page'))

@blueprint.route("/logout")
def logout_page():
    stop_session()
    return flask.redirect(flask.url_for('.main_page'))

@blueprint.route("/hello_user")
@in_user_context
def hello_user_page():
    return flask.render_template('demo/hello_user.html')

@blueprint.route("/db")
@with_db_session
def db_test_page(db):
    db.execute('SELECT * FROM demo')
    data = {row.key:row.value for row in db.fetchall()}
    return flask.render_template("demo/db_test.html", data=data)