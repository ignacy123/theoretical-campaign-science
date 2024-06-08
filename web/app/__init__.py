#!/usr/bin/env python3
import flask, json, sys
from .users import blueprint as users_blueprint
from .demo import blueprint as demo_blueprint
from .blog import blueprint as blog_blueprint, main_page as blog_main_page
from .database import database_setup

app = flask.Flask(__name__)
app.config.from_file('config.json', load=json.load)

database_setup(app)
app.register_blueprint(users_blueprint)
app.register_blueprint(demo_blueprint, url_prefix='/test')
app.register_blueprint(blog_blueprint, url_prefix='/blog')

if (main_redirect := app.config.get('MAIN_REDIRECT')) is not None:
    app.add_url_rule('/', 'main', lambda :flask.redirect(f'/{main_redirect}'))