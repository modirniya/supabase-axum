# Supabase-Axum Template

A Rust backend template built with Axum and Supabase integration. This template provides a solid foundation for building APIs with authentication, database integration, and structured development.

## Features

- JWT authentication with Supabase
- PostgreSQL database integration with SQLx
- User profile management
- Role-based access control
- Structured error handling

## Prerequisites

- Rust 1.75 or later (2024 edition)
- PostgreSQL database (or Supabase instance)
- Docker (optional, for containerization)

## Setup

1. Clone the repository:

```bash
git clone https://github.com/yourusername/supabase-axum.git
cd supabase-axum
```

2. Create a `.env` file with your configuration:

```
DATABASE_URL=postgres://postgres:password@localhost:5432/yourdb
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_JWKS_URL=https://your-project.supabase.co/auth/v1/jwks
```

3. Set up the database schema:

```bash
psql -U postgres -d yourdb -f src/db/schema.sql
```

Or apply the schema directly in your Supabase SQL editor.

4. Build and run the application:

```bash
cargo build
cargo run
```

## Development

### SQLx Offline Mode

This project supports SQLx offline mode for development without a database connection. To update the SQLx query data:

```bash
cargo sqlx prepare --database-url "postgres://postgres:password@localhost:5432/yourdb"
```

### Testing

Run the test suite:

```bash
cargo test
```

### Building for Production

```bash
cargo build --release --features prod
```

## Docker

Build the Docker image:

```bash
docker build -t supabase-axum .
```

Run the container:

```bash
docker run -p 3000:3000 --env-file .env supabase-axum
```

## Project Structure

- `src/auth/`: JWT authentication and user context
- `src/db/`: Database connection, models, and repositories
- `src/routes/`: API route handlers
- `src/main.rs`: Application entry point

## License

MIT 