import flask
from .database import with_db_session

blueprint = flask.Blueprint("admin", __name__)


@with_db_session
def db_get_intersection(db):
    db.execute("SELECT * FROM intersection")
    return [row.username for row in db.fetchall()]


@blueprint.route("/")
def main_page():
    return flask.redirect(flask.url_for('.panel_page'))

@blueprint.route("/panel")
def panel_page():
    intersection = db_get_intersection()
    return flask.render_template('admin/panel.html', intersection=intersection)