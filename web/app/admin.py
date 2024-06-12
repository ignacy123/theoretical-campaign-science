import flask
from .database import with_db_session
from .users import in_admin_context, set_admin_session, stop_admin_session, is_in_admin_session, admin_password_correct

blueprint = flask.Blueprint("admin", __name__)


@with_db_session
def db_get_intersection(db):
    db.execute("SELECT * FROM intersection")
    return [row.username for row in db.fetchall()]


@blueprint.route("/")
def main_page():
    if is_in_admin_session():
        return flask.redirect(flask.url_for('.panel_page'))
    else:
        return flask.redirect(flask.url_for('.login_page'))

@blueprint.route("/login", methods=["GET"])
def login_page():
    return flask.render_template("admin/login.html")

@blueprint.route("/login", methods=["POST"])
def login_request():
    try:
        password = flask.request.form["password"]
    except KeyError:
        flask.abort(400) # 400: Bad Request
    
    if not admin_password_correct(password):
        flask.flash("Password is incorrect")
        return flask.redirect(flask.url_for(".login_page"))
    
    set_admin_session()
    return flask.redirect(flask.url_for(".panel_page"))

@blueprint.route("/logout", methods=["POST"])
def logout_request():
    stop_admin_session()
    return flask.redirect(flask.url_for(".login_page"))

@blueprint.route("/panel")
@in_admin_context
def panel_page():
    intersection = db_get_intersection()
    return flask.render_template('admin/panel.html', intersection=intersection)