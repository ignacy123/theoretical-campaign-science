#!/usr/bin/env python3
import flask, json, sys
from .users import blueprint as users_blueprint
from .demo import blueprint as demo_blueprint
from .blog import blueprint as blog_blueprint
from .shop import blueprint as shop_blueprint
from .admin import blueprint as admin_blueprint
from .database import database_setup

app = flask.Flask(__name__)
app.config.from_file('config.json', load=json.load)

database_setup(app)
app.register_blueprint(users_blueprint)

blueprints = {
    "demo"  : demo_blueprint,
    "blog"  : blog_blueprint,
    "shop"  : shop_blueprint,
    "admin" : admin_blueprint
}
for feature in app.config['FEATURES']:
    try:
        app.register_blueprint(blueprints[feature], url_prefix=f'/{feature}')
        print(f"Loaded feature: {feature}", file=sys.stderr)
    except KeyError:
        sys.exit(f"ERROR: No such feature: {feature}")

if (feature := app.config.get('MAIN_FEATURE')) is not None:
    app.add_url_rule('/', 'main', lambda :flask.redirect(f'/{feature}'))