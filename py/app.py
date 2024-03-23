from flask import Flask, render_template, session, redirect, url_for, abort, request, send_from_directory
from functools import wraps
from pymongo.mongo_client import MongoClient
from pymongo.server_api import ServerApi
import os

app = Flask(__name__)
# python -c 'import os; print(os.urandom(16))'
app.secret_key = b'S\xb3\xd9\xe4\xb7\xed\xceM\x00\xf0\xd89\xd9f\x8f\xa6'
# Set the domain name
# app.config['SERVER_NAME'] = 'a35606495bb502.lhr.life'
# Databse
# username_db = os.environ['usernamedb']
# password_db = os.environ['passworddb']

# uri = "mongodb+srv://{}:{}@cluster0.h3wdkqr.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".format(
#     username_db, password_db)
uri = 'mongodb://localhost:27017/'
# Create a new client and connect to the server
client = MongoClient(uri, server_api=ServerApi('1'))
mydb = client.DBNAMES

# mycol = mydb["COLLECTIONNAME"]


# Decorators
def login_required(func):

  @wraps(func)
  def wrapper(*args, **kwargs):
    if 'logged_in' in session:
      return func(*args, **kwargs)
    else:
      return redirect(url_for('notlogin'))  #, 401

  return wrapper


def already_login(func):

  @wraps(func)
  def wrapper(*args, **kwargs):
    if 'logged_in' in session:
      return redirect(url_for('dashboard'))
    else:
      return func(*args, **kwargs)
      # return render_template('home.html')

  return wrapper

@app.route('/favicon.ico')
def favicon():
    return send_from_directory(os.path.join(app.root_path, 'static'),
                               'favicon.ico', mimetype='image/vnd.microsoft.icon')

# Routes
from user import routes
from apps.csc import csc
# maintenance_mode = True
maintenance_mode = False
if maintenance_mode:

  @app.route('/', defaults={'path': ''})
  @app.route('/<path:path>')
  def maintenance(path):
    return render_template('maintenance.html'), 503
else:

  @app.route('/')
  @already_login
  def home():
    return render_template('home.html')

  @app.errorhandler(404)
  def not_found(e):
    return render_template("404.html"), 404

  @app.route('/notlogin')
  def notlogin():
    return render_template('notlogin.html'), 401

  @app.route('/dashboard/')
  @login_required
  def dashboard():
    return render_template('dashboard.html')

  @app.route('/test/')
  def test():
    return render_template('test.html')

  @app.route('/version/')
  def version():
    return 'Version 0.0.0'


if __name__ == "__main__":
    # app.debug = True
    app.run(host='0.0.0.0', port=8080)
