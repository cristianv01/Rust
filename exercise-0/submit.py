#!/usr/bin/python3

import getpass
import json
import requests
import shutil
import sys
import tempfile
from pathlib import Path

SEMESTER = "fall2024rust"
API_URL = f"https://rust.cs.vt.edu/autograder_api/{SEMESTER}"

def submit_file(tag, file, api_url, cookies):
    r = requests.post(api_url + "/upload/" + str(tag), cookies=cookies, files={"uploadedfile": open(file, "rb")},timeout=3.0)
    return r


def authenticate(api_url, user, password):
    r = requests.post(api_url + "/login", json={'username': user, 'password': password},timeout=1.0)
    r.raise_for_status()
    if r.status_code == requests.codes.ok:
        return r.cookies
    # could not authenticate
    return None


if __name__ == "__main__":
    project_id = "ex1"
    exercise_folder = Path("exercises")
    if not exercise_folder.exists() or not exercise_folder.is_dir():
        print("The folder `exercises` does not exist, are you running the script in the right directory?")
        sys.exit(1)

    with tempfile.TemporaryDirectory() as temp_dir:
        tar_file = Path(temp_dir).joinpath("submission")
        tar_file = shutil.make_archive(str(tar_file), "gztar", base_dir=str(exercise_folder))
        print(f"Tar file created: {tar_file}")

        user = input("Enter your VT username > ")
        print("Enter the SLO password for user {} when prompted".format(user))
        password = getpass.getpass()

        cookies = authenticate(API_URL, user, password)
        if not cookies:
            print(f"Authentication failed for user {user}")
            sys.exit(1)
        r = submit_file(project_id, tar_file, API_URL, cookies)

        print(f"Server responded with {r.text}")

        r = json.loads(r.text)
        if r["upload"] == 1:
            print("Success uploading the submission, but the submission has not been checked for completeness or autograded.")
            print(f"Once it is, autograder results will be at\nhttps://rust.cs.vt.edu/cs3984/{SEMESTER}/submission/{project_id}/{r['uuid']}")
            print(f"You can see the status of all your submissions at: https://rust.cs.vt.edu/cs3984/{SEMESTER}/submissions")
        else:
            print(f"Submission upload failed: {r['msg']}")

