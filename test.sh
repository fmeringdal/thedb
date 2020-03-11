WEBSERVER="webserver"
STORAGEENGINE="storageengine"

projects=($WEBSERVER $STORAGEENGINE)

for project in "${projects[@]}"
do
    cargo test --manifest-path "./projects/${project}/Cargo.toml"
done