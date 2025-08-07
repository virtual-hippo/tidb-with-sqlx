# tidb-with-sqlx

A Rust application demonstrating async database operations with SQLx, supporting both local MySQL and TiDB connections with TLS/SSL support.

## Features

- Async database operations using SQLx
- TLS/SSL connection support for secure database access
- User CRUD operations (Create, Read)
- Japanese character support in test data
- Docker Compose setup for local development
- Database migration support
- Configurable environment for local and Docker deployments

## Dependencies

- **SQLx**: Async SQL toolkit with MySQL support
- **Tokio**: Async runtime for Rust
- **Chrono**: Date and time handling with serde support
- **Serde**: Serialization framework

## Project Structure

```
├── src/
│   └── main.rs          # Main application with database operations
├── migrations/
│   └── 20250807235854_start.up.sql  # Database schema migration
├── certs/
│   └── local-ca.pem     # CA certificate for TLS connections
├── Cargo.toml           # Rust dependencies and project config
├── Makefile.toml        # Build tasks and automation
├── compose.yaml         # Docker Compose for MySQL setup
└── rustfmt.toml         # Rust formatting configuration
```

## Database Operations

The application demonstrates:
- **User Model**: Struct with id, name, email, and timestamps
- **Data Insertion**: Inserts Japanese test users into the database
- **Data Retrieval**: Fetches and displays all users from the database
- **Connection Management**: Secure database connections with optional TLS

## Setup and Usage

### Prerequisites

- Rust toolchain
- Docker and Docker Compose (for local MySQL)
- `cargo-make` for task automation

### Local Development with Docker

1. Start the MySQL database:
```bash
cargo make compose-up-db
```

2. Run database migrations:
```bash
cargo make migrate
```

3. Run the application:
```bash
cargo make run
```

### TiDB Connection

1. Set up environment variables in `.env` file:
```bash
DATABASE_HOST=your-tidb-host
DATABASE_PORT=4000
DATABASE_USERNAME=your-username
DATABASE_PASSWORD=your-password
DATABASE_NAME=your-database
CA_CERT_NAME=ca-cert.pem
CERT_DIR=./certs
```

2. Run with TiDB:
```bash
cargo make run-with-tidb
```

## Available Tasks

- `cargo make run` - Run application with local MySQL
- `cargo make run-with-tidb` - Run application with TiDB
- `cargo make migrate` - Run database migrations
- `cargo make clippy` - Run Rust linter
- `cargo make fmt` - Check code formatting
- `cargo make compose-up-db` - Start MySQL container
- `cargo make compose-down` - Stop Docker services

## Environment Configuration

The application supports two environments:
- **Local**: Connects to `localhost:3306` for Docker MySQL
- **TiDB**: Uses `.env` file configuration with TLS support

SSL/TLS is automatically enabled when `CA_CERT_NAME` and `CERT_DIR` environment variables are set.