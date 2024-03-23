from flask import Flask, jsonify, request, session, redirect
from passlib.hash import pbkdf2_sha256
from werkzeug.utils import redirect
from app import mydb
import uuid
from bson.json_util import dumps, loads 


class User:

  def start_session(self, user):
    del user['password']
    session['logged_in'] = True
    session['user'] = user
    return jsonify(user), 200

  def signup(self):
    # print(request.form)

    # Create the user object
    uid = uuid.uuid4().hex
    user = {
        "_id": uid,
        "username": request.form.get('username'),
        "password": request.form.get('password')
    }

    avatar = {
        "_id": uid,
        "username": request.form.get('username'),
        "avatar": self.avatar()
    }

    # Encrypt the password
    user['password'] = pbkdf2_sha256.encrypt(user['password'])

    # Check for existing username
    if mydb.COLLECTIONNAME.find_one({'username': user['username']}):
      return jsonify({"error": "Username already exists"}), 400

    if mydb.COLLECTIONNAME.insert_one(user):
      mydb.AVATAR.insert_one(avatar)
      return self.start_session(user)

    return jsonify({"error": "Signup failed"}), 400

  def signout(self):
    session.clear()
    return redirect('/')

  def login(self):
    # print(request.form)
    username = request.form.get('username')
    password = request.form.get('password')

    if not (username and password):
      return jsonify({"error": "Username or password missing"}), 400

    user = mydb.COLLECTIONNAME.find_one(
        {'username': request.form.get('username')})

    if user and pbkdf2_sha256.verify(password, user['password']):
      return self.start_session(user)

    return jsonify({"error": "Invalid login credentials"}), 401

  def delete(self):
    if session['user'] is not None and 'username' in session['user']:
        mydb.COLLECTIONNAME.delete_one({'username': session['user']['username']})
        mydb.AVATAR.delete_one({'username': session['user']['username']})
        session.clear()
        return redirect('/')
    else:
        return jsonify({"error": "User information not found"}), 400

  def avatar(self):
    import base64
    # Convert the image to base64 format
    with open("test.png", "rb") as f:
      encoded_image = base64.b64encode(f.read())

    return encoded_image

  def getAvatar(self,path):
    if mydb.AVATAR.find_one({'username': str(path)}):
      data_request = mydb.AVATAR.find_one({'username': str(path)})
      return jsonify(data_request['avatar'].decode("utf-8")), 200
    else:
      return jsonify({"error": "Avatar for {} user is not found".format(str(path))})
