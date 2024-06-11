#!/usr/bin/env python3
import flask, json, sys
from .users import blueprint as users_blueprint
from .demo import blueprint as demo_blueprint
from .blog import blueprint as blog_blueprint
from .shop import blueprint as shop_blueprint
from .database import database_setup

app = flask.Flask(__name__)
app.config.from_file('config.json', load=json.load)

database_setup(app)
app.register_blueprint(users_blueprint)
app.register_blueprint(demo_blueprint, url_prefix='/demo')
if 'blog' in app.config['FEATURES']:
    app.register_blueprint(blog_blueprint, url_prefix='/blog')
if 'shop' in app.config['FEATURES']:
    app.register_blueprint(shop_blueprint, url_prefix='/shop')

if (main_redirect := app.config.get('MAIN_REDIRECT')) is not None:
    app.add_url_rule('/', 'main', lambda :flask.redirect(f'/{main_redirect}'))