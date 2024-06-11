import flask

app = flask.Flask(__name__)

@app.before_request
def redirect_to_https():
    url = flask.request.url
    if url[:4] == 'http':
        return flask.redirect('https' + url[4:], 301)
    else:
        flask.abort(400) # should not happen
