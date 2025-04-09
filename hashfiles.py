#!/usr/bin/env python3
import hashlib
import os
import base64

TRUNK_STAGING_DIR = os.environ["TRUNK_STAGING_DIR"]
name = "pf1_spell_search"

with open(f"{TRUNK_STAGING_DIR}/{name}.js", "rb") as f:
    sha = hashlib.sha256()
    sha384 = hashlib.sha384()
    data = f.read()
    sha.update(data)
    sha384.update(data)
    jshash = sha.hexdigest()[:8]
    fulljshash_old = sha384.digest()
    fulljshash_old = "sha384-" + base64.b64encode(fulljshash_old).decode()

with open(f"{TRUNK_STAGING_DIR}/{name}_bg.wasm", "rb") as f:
    sha = hashlib.sha256()
    sha.update(f.read())
    wasmhash = sha.hexdigest()[:8]

with open(f"{TRUNK_STAGING_DIR}/{name}.js") as f:
    data = f.read()
data = data.replace("{name}_bg.wasm", f"{name}_bg-{wasmhash}.wasm")
with open(f"{TRUNK_STAGING_DIR}/{name}.js", "w") as f:
    f.write(data)

with open(f"{TRUNK_STAGING_DIR}/{name}.js", "rb") as f:
    sha384 = hashlib.sha384()
    data = f.read()
    sha384.update(data)
    fulljshash_new = sha384.digest()
    fulljshash_new = "sha384-" + base64.b64encode(fulljshash_new).decode()

with open(f"{TRUNK_STAGING_DIR}/index.html") as f:
    data = f.read()
data = data.replace("{name}.js", f"{name}-{jshash}.js")
data = data.replace("{name}_bg.wasm", f"{name}_bg-{wasmhash}.wasm")
data = data.replace(fulljshash_old, fulljshash_new)
with open(f"{TRUNK_STAGING_DIR}/index.html", "w") as f:
    f.write(data)

with open(f"{TRUNK_STAGING_DIR}/sw.js") as f:
    data = f.read()
data = data.replace("{name}.js", f"{name}-{jshash}.js")
data = data.replace("{name}_bg.wasm", f"{name}_bg-{wasmhash}.wasm")
with open(f"{TRUNK_STAGING_DIR}/sw.js", "w") as f:
    f.write(data)

os.rename(f"{TRUNK_STAGING_DIR}/{name}.js", f"{TRUNK_STAGING_DIR}/{name}-{jshash}.js")
os.rename(f"{TRUNK_STAGING_DIR}/{name}_bg.wasm", f"{TRUNK_STAGING_DIR}/{name}_bg-{wasmhash}.wasm")