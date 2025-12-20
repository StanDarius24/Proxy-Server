from flask import Flask, request, jsonify

app = Flask(__name__)

@app.route("/", defaults={"path": ""})
@app.route("/<path:path>")
def catch_all(path):
    return jsonify({"path": "/" + path}), 200

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=80)
