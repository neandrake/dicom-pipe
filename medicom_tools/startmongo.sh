# Start a local mongo with persisted data storage, for testing medicom_tools.
docker run --rm --publish 27017:27017 --volume dbdata:/data/db -it mongo:7.0

