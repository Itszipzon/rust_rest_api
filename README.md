# Rust REST API

This project is a RESTful API built using **Rust**, aimed at deepening understanding of both the Rust programming language and REST API development. It connects to a **PostgreSQL** database for data persistence.

## Environment Variables

The application expects the following environment variables to be defined:
```
DATABASE_USERNAME = {Username for the PostgreSQL database}
DATABASE_PASSWORD = {Password for the PostgreSQL user}
DATABASE_HOST = {Hostname or IP address of the database server}
DATABASE_PORT = {Port number for the PostgreSQL server (default is 5432)}
DATABASE_NAME = {Name of the PostgreSQL database}

JWT_SECRET = {Secret key used to sign and verify JWT tokens}
```

💡 You can define these in a `.env` file for local development.
