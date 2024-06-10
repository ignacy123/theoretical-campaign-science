import flask
from .users import in_user_context, set_session, validate_login
from .database import with_db_session
from psycopg2.errors import UniqueViolation

blueprint = flask.Blueprint("shop", __name__)


@with_db_session
def db_already_purchased(db, username:str) -> bool:
    db.execute("""
               SELECT * FROM sales
               WHERE username = %s
               """, (username,))
    return db.fetchone() is not None

@with_db_session
def db_add_purchase(db, username:str):
    try:
        db.execute("""
                INSERT INTO sales (username)
                VALUES (%s)
                """, (username,))
        return True
    except UniqueViolation:
        return False


@blueprint.route("/")
def main_page():
    return flask.render_template('shop/main.html')

@blueprint.route("/login", methods=["POST"])
def login_request():
    try:
        username = flask.request.form['login']
        validate_login(username)
    except (KeyError, ValueError):
        flask.abort(400) # 400: Bad Request

    set_session(username)
    return flask.redirect(flask.url_for(".offer_page"))

@blueprint.route("/offer")
@in_user_context
def offer_page():
    data = {}
    data['can_buy'] = not db_already_purchased(flask.g.user)
    return flask.render_template("shop/offer.html", **data)

@blueprint.route("/purchase", methods=["POST"])
@in_user_context
def purchase_request():
    if not db_add_purchase(flask.g.user):
        return flask.abort(418) # 418: I'm a teapot
    return flask.redirect(flask.url_for(".offer_page"))