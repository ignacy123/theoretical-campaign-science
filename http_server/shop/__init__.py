#!/usr/bin/env python3
import flask, json
from .users import blueprint as users_blueprint
from .demo import blueprint as demo_blueprint
from .database import database_setup

app = flask.Flask(__name__)
app.config.from_file('config.json', load=json.load)

database_setup(app)
app.register_blueprint(users_blueprint)
app.register_blueprint(demo_blueprint, url_prefix='/test')