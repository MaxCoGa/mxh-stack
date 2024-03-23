from flask import Flask
from app import app, login_required
from user.models import User


account_creation = True
account_deletion = True

@app.route('/user/signup', methods=['POST'])
def signup():
  return User().signup()


@app.route('/user/signout')
def signout():
  return User().signout()


@app.route('/user/login', methods=['POST'])
def login():
  return User().login()

@app.route('/user/delete')
@login_required
def delete():
  return User().delete()

@app.route('/user/avatar/', defaults={'path': ''})
@app.route('/user/avatar/<path:path>', methods=['GET'])
def getAvatar(path):
  return User().getAvatar(path)
