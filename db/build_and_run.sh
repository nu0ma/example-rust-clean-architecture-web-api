docker build . --no-cache -t db 
docker run --rm --name db  -p 5432:5432 -e POSTGRES_PASSWORD=password db
