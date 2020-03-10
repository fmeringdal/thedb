webserver="webserver"
storageengine="storageengine"

if [ $# -eq 0 ]
  then
    echo "No arguments supplied"
    exit
fi

if [ $1 == $webserver ] || [ $1 == $storageengine ]
then
    cargo run --manifest-path "./projects/${1}/Cargo.toml"
else
    echo "Supplied project name is not valid"
fi
