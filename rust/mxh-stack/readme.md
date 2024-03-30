cargo run



self-signed cert

openssl req -x509 -newkey rsa:4096 -keyout ./cert/key-pass.pem -out ./cert/cert-pass.pem -sha256 -days 365 -subj '/CN=localhost'


localhost to web test

ssh -R 80:localhost:8080 nokey@localhost.run

ssh -R 80:localhost:8443 nokey@localhost.run