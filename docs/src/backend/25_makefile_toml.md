# Bonus: Makefile.toml

Final section of the backend part of the workshop. 

Create a file in the root of the project called `Makefile.toml` with the following content:

```toml
# project tasks
[tasks.api-run]
workspace = false
env = { RUST_LOG="info" }
install_crate = "cargo-shuttle"
command = "cargo"
args = ["shuttle", "run"]

[tasks.front-serve]
workspace = false
cwd = "./front"
install_crate = "dioxus-cli"
command = "dioxus"
args = ["serve"]

[tasks.front-build]
workspace = false
script_runner = "@shell"
script = '''
# shuttle issue with static files
# location is different depending on the environment
rm -rf api/shuttle/static static
mkdir api/shuttle/static
mkdir static
cd front
dioxus build --release
# local development 
cp -r dist/* ../api/shuttle/static
# production
cp -r dist/* ../static
'''

# local db
[tasks.db-start]
workspace = false
script_runner = "@shell"
script = '''
docker run -d --name devbcn-workshop -p 5432:5432 -e POSTGRES_PASSWORD=postgres -e POSTGRES_USER=postgres -e POSTGRES_DB=devbcn postgres
'''

[tasks.db-stop]
workspace = false
script_runner = "@shell"
script = '''
docker stop postgres
docker rm postgres
'''

# general tasks
[tasks.clippy]
workspace = false
install_crate = "cargo-clippy"
command = "cargo"
args = ["clippy"]

[tasks.format]
clear = true
workspace = false
install_crate = "rustfmt"
command = "cargo"
args = ["fmt", "--all", "--", "--check"]
```

It may be useful, specially for building the frontend.

```admonish info
Learn more about [cargo-make](https://sagiegurari.github.io/cargo-make/), [clippy](https://doc.rust-lang.org/stable/clippy/index.html) and [rustfmt](https://github.com/rust-lang/rustfmt).
```

Commit this change:

```bash
git add .
git commit -m "add Makefile.toml"
```
