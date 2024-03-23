# mxh-stack

python -m venv mxh-stack-venv
Set-ExecutionPolicy Unrestricted -Scope Process 
.\mxh-stack-venv\Scripts\Activate.ps1 
Set-ExecutionPolicy Default -Scope Process
pip install -r requirements.txt

run with: 
flask --app app run --host=0.0.0.0 --debug
ssh -R 80:192.168.2.156:5000 nokey@localhost.run

deactivate

mongodb needs to be running