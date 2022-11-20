CREATE USER numa WITH PASSWORD 'password' SUPERUSER;
CREATE DATABASE example owner numa encoding 'UTF8';
GRANT ALL PRIVILEGES ON DATABASE example To numa;
\c example;
CREATE SCHEMA example;
SET client_encoding = 'UTF-8';