from psycopg2 import connect
from psycopg2.extras import NamedTupleCursor
from queue import Queue
from functools import wraps
from flask import current_app, g

def database_setup(app):
    config = {
        "host": app.config["DB_HOST"],
        "port": app.config["DB_PORT"],
        "dbname": app.config["DB_NAME"],
        "user": app.config["DB_USER"]
    }

    conns = app.config["DB_CONNECTIONS"]
    queue = Queue(maxsize=conns)
    app.extensions["data"] = config, queue

    for _ in range(conns):
        queue.put(None)

# Decorator meaning that the function requires database connection.
# The cursor object is passed as the first argument
def with_db_session(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        def call():
            with g.conn.cursor(cursor_factory=NamedTupleCursor) as c:
                return func(c, *args, **kwargs)

        def call_new():
            config, queue = current_app.extensions["data"]
            conn = queue.get()
            try:
                if not conn or conn.closed:
                    conn = connect(**config)
                with conn:
                    g.conn = conn
                    return call()
            finally:
                g.pop("conn")
                queue.put(conn)

        if "conn" in g:
            return call()
        else:
            return call_new()

    return wrapper
