from pymongo.mongo_client import MongoClient
from pymongo.server_api import ServerApi
import os

username_db = os.environ['usernamedb']
password_db = os.environ['passworddb']

uri = "mongodb+srv://{}:{}@cluster0.h3wdkqr.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".format(
    username_db, password_db)

# Create a new client and connect to the server
client = MongoClient(uri, server_api=ServerApi('1'))

# Send a ping to confirm a successful connection
try:
  client.admin.command('ping')
  print("Pinged your deployment. You successfully connected to MongoDB!")
except Exception as e:
  print(e)

mydb = client["DBNAME"]
print(client.list_database_names())

mycol = mydb["COLLECTIONNAME"]
print(mydb.list_collection_names())

username = input("Enter your username: ")
password = input("Enter your password: ")

mydict = {"username": username, "password": password}
x = mycol.insert_one(mydict)
print(x.inserted_id)

from pymongo import MongoClient
from pymongo.server_api import ServerApi
import os

username_db = os.environ.get('usernamedb', "default_value")
password_db = os.environ.get('passworddb', "default_value")
uri = "mongodb+srv://{}:{}@cluster0.h3wdkqr.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".format(
    username_db, password_db)


def get_db_handle():
  # fill_color='#f4e8ff', back_color='#120024'
  client = MongoClient(uri, server_api=ServerApi('1'))
  db_handle = client['DBNAME']
  return db_handle, client

  # from pymongo import MongoClient
  # def get_db_handle(db_name, host, port, username, password):

  #  client = MongoClient(host=host,
  #                       port=int(port),
  #                       username=username,
  #                       password=password
  #                      )
  #  db_handle = client['db_name']
  #  return db_handle, client
