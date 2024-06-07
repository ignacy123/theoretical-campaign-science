import flask
from .users import in_user_context, login, logout

blueprint = flask.Blueprint("blog", __name__)

@blueprint.route("/")
def main_page():
    return flask.render_template('blog/main.html')

@blueprint.route("/login", methods=["POST"])
def login_query():
    # input validation
    try:
        username = flask.request.form['login']
    except KeyError:
        flask.abort(400) # 400: Bad Request
    if not username.isalnum():
        flask.abort(418) # 418: I'm a Teapot

    login(username)
    return flask.redirect(flask.url_for(".content_page"))

@blueprint.route("/content")
@in_user_context
def content_page():
    return flask.render_template("blog/content.html")