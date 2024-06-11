import flask
from .users import in_user_context, set_session, validate_login
from .database import with_db_session

blueprint = flask.Blueprint("blog", __name__)


def validate_ad_id(ad_id:str):
    if not (1 <= len(ad_id) <= 32):
        raise ValueError

@with_db_session
def register_ad_click(db, username:str, ad_id:str):
    db.execute("""
               INSERT INTO clicks (username, ad)
               VALUES (%s, %s)
               ON CONFLICT DO NOTHING
               """, (username, ad_id))


@blueprint.route("/")
def main_page():
    return flask.render_template('blog/main.html')

@blueprint.route("/login", methods=["POST"])
def login_request():
    try:
        username = flask.request.form['login']
        validate_login(username)
    except (KeyError, ValueError):
        flask.abort(400) # 400: Bad Request

    set_session(username)
    return flask.redirect(flask.url_for(".content_page"))

@blueprint.route("/content")
@in_user_context
def content_page():
    return flask.render_template("blog/content.html")

@blueprint.route("/ad_click")
@in_user_context
def ad_click_request():
    #input validation
    try:
        link = flask.request.args['link']
        ad_id = flask.request.args['id']
        validate_ad_id(ad_id)
    except (KeyError, ValueError):
        flask.abort(400) # 400: Bad Request
    
    register_ad_click(flask.g.user, ad_id)

    return flask.redirect(link)