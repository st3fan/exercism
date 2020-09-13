

import json


class RestAPI:

    def __init__(self, database=None):
        self.database = database

    def get(self, url, payload=None):
        if url == "/users":
            return json.dumps({"users": self._get_users()})

    def post(self, url, payload=None):
        if url == "/add":
            user = json.loads(payload)
            return json.dumps(self._add_user(user["user"]))

    def _get_users(self):
        return self.database["users"]

    def _add_user(self, name):
        if name not in self.database["users"]:
            self.database["users"][name] = {"name": name, "owes": {}, "owed_by": {}, "balance": 0.0}
        return self.database["users"][name]
