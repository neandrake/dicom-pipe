REM start a local mongo with persisted data storage, for testing dcmpipe_cli
docker run --rm --publish 27017:27017 --mount source=mongodata,target=/data/db -it mongo:7.0

