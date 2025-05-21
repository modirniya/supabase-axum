# Supabase-Axum Template Development Plan

This document outlines a multi-stage developmental plan for the 'supabase-axum-template' Rust backend, focusing on logic, relationships, entities, and necessary libraries for internal JWT verification and purpose-agnostic service development.

## Phase 1: Project Initialization & Foundation

### Logic
Establish project structure, manage dependencies, configure environment.

### Relationships
Project root to module structure.

### Entities
None at this stage.

### Libraries
- **cargo**: Project manager.
- **axum**: Web framework.
- **tokio**: Asynchronous runtime.
- **serde**: Serialization/deserialization.
- **dotenvy** or **config**: Environment variable management.

### Interactions
None beyond initial setup.

## Phase 2: Authentication & Authorization Core

### 2.1 JWKS Fetching and Caching

#### Logic
Asynchronously fetch JSON Web Key Set (JWKS) from Supabase, parse it, and cache for subsequent use. Handle potential network errors and refresh logic.

#### Relationships
Application startup -> JWKS fetching service/function.

#### Entities
JwkSet (struct representing the fetched JWKS).

#### Libraries
- **reqwest**: HTTP client for fetching the JWKS.
- **jsonwebtokens-jwk**: Parsing JWKS.
- **tokio::sync::OnceCell** (for caching the JWKS).

#### Interactions
Application starts, makes an HTTP GET request to Supabase JWKS endpoint, stores the result.

### 2.2 JWT Validation Middleware

#### Logic
Intercept incoming HTTP requests, extract the JWT from the Authorization header, and validate its signature and standard claims (exp, iss, aud) against the cached JWKS.

#### Relationships
Incoming Request -> Axum Middleware -> Handler.

#### Entities
AuthError (specific error types for JWT validation failures).

#### Libraries
- **jsonwebtoken**: JWT decoding and signature verification.
- **axum::extract::FromRequestParts**: For creating custom middleware extractors.

#### Interactions
Middleware receives request, attempts JWT validation; if invalid, returns early with an error response (401/403).

### 2.3 User Context Extraction

#### Logic
Upon successful JWT validation, extract relevant user identity information (e.g., user_id, email, roles) from the JWT claims and make it available to downstream handlers.

#### Relationships
Validated JWT -> AuthUser struct -> Request Extensions/State.

#### Entities
AuthUser (struct to hold decoded JWT claims: id, email, role, etc.).

#### Libraries
- **jsonwebtoken**: For accessing claims after decoding.
- **axum::extract::Extension**: For inserting AuthUser into request context.

#### Interactions
Middleware populates AuthUser and adds it to the request; handlers can then extract AuthUser to access user data.

## Phase 3: Database Integration & Basic Data Model

### 3.1 Database Connection Management

#### Logic
Initialize a database connection pool at application startup, manage its lifecycle, and provide it as shared application state.

#### Relationships
Application startup -> Database Pool.

#### Entities
sqlx::PgPool (the database connection pool).

#### Libraries
- **sqlx**: For creating and managing the PostgreSQL connection pool.
- **tokio**: For asynchronous pool management.

#### Interactions
Application initializes, creates database pool, makes it available to all handlers.

### 3.2 Basic User Model and CRUD Operations

#### Logic
Define a Rust struct that maps to a basic users table in the database (e.g., a public `profiles` table linked to `auth.users`). Implement basic Create, Read, Update, Delete (CRUD) operations for this model. This demonstrates interaction with a database beyond Supabase-managed tables if needed.

#### Relationships
AuthUser (from JWT) -> User (database model, e.g., `Profile`).

#### Entities
User (struct representing a database user, e.g., id, email, created_at, custom profile fields).

#### Libraries
- **sqlx**: For executing database queries.
- **serde**: For serializing/deserializing database results to/from User struct.

#### Interactions
Handlers receive AuthUser, use its id to query/manipulate User data in the custom database table.

### 3.3: Sample Role-Based Endpoints (Dummy Service)

#### Logic
Implement simple "echo" API endpoints to demonstrate role-based authorization based on claims in the Supabase JWT.
- An `/api/echo` endpoint accessible by any authenticated user.
- An `/api/premium_echo` endpoint accessible only by authenticated users possessing a "premium" role.
This setup will serve as a placeholder, guiding developers on how to implement more complex services with role-based access control (RBAC). Assumes user roles (e.g., from `auth.users.role` or custom claims) are present in the Supabase JWT.

#### Relationships
Authenticated Request -> Role Authorization (performed in-handler for simplicity, or via dedicated middleware) -> Handler.
`AuthUser` (from JWT, containing parsed role) -> Authorization Logic within Handler.
API Routes -> Specific Handlers.

#### Entities
- **`Role` enum**: (e.g., `#[derive(Debug, Clone, PartialEq, serde::Deserialize)] enum Role { User, Premium }`) to represent user roles, parsed from JWT claims and stored in `AuthUser`.
- **`EchoRequest` struct**: (e.g., `#[derive(serde::Deserialize)] struct EchoRequest { message: String }`) for incoming echo data.
- **`EchoResponse` struct**: (e.g., `#[derive(serde::Serialize)] struct EchoResponse { echoed_message: String, user_id: String, role: String }`) for outgoing echo data.
- ***Modification***: `AuthUser` struct (from Phase 2.3) is updated to include the parsed `Role` enum.

#### Libraries
- None new beyond existing `axum`, `serde`. Internal modules for `Role` enum and request/response DTOs will be created.

#### Interactions
1.  Update `AuthUser` struct (Phase 2.3) to parse the relevant JWT claim (e.g., 'role', 'user_roles', or a custom claim from `app_metadata`) into the `Role` enum.
2.  Implement `echo_handler` for `POST /api/echo`:
    -   Extracts `AuthUser` via `axum::extract::Extension`.
    -   Accepts `Json<EchoRequest>`.
    -   Returns `Json<EchoResponse>` including user's ID and role.
3.  Implement `premium_echo_handler` for `POST /api/premium_echo`:
    -   Extracts `AuthUser`.
    -   Performs a role check (e.g., `if user.role == Role::Premium`).
    -   If unauthorized, returns an `AppError` resulting in a 403 Forbidden response.
    -   Accepts `Json<EchoRequest>`.
    -   Returns `Json<EchoResponse>` with a premium-specific message.
4.  Add these new routes to the Axum router, ensuring they are protected by the JWT validation middleware (from Phase 2.2).
5.  *Note in Documentation*: For applications with multiple roles or complex permissions, advise transitioning from in-handler role checks to dedicated Axum middleware or extractor guards for better separation of concerns and reusability (DRY principle).

## Phase 4: Error Handling & Logging

### 4.1 Centralized Error Handling

#### Logic
Define a custom, unified error type (AppError) that encapsulates all possible application errors. Implement conversion from various error sources (e.g., sqlx::Error, jsonwebtoken::Error, custom auth errors) to AppError. Implement IntoResponse for AppError to convert it into standardized HTTP error responses (e.g., JSON with status code).

#### Relationships
All application components -> AppError.

#### Entities
Custom AppError enum with variants for different error scenarios (e.g., Unauthorized, Forbidden, NotFound, DatabaseError, ValidationError).

#### Libraries
- **thiserror**: Derive macros for Error trait and automatic From implementations.
- **axum::response::IntoResponse**: For converting AppError to HTTP responses.
- **serde_json**: For serializing error responses to JSON.

#### Interactions
Any function returning a Result can use AppError; middleware/handlers automatically convert AppError into a user-friendly HTTP response.

### 4.2 Structured Logging Setup

#### Logic
Configure a structured logging system to capture application events, errors, and debugging information. Integrate logging throughout the application.

#### Relationships
Application components -> Logging system.

#### Entities
Log records (structured key-value pairs).

#### Libraries
- **tracing**: Core structured logging library.
- **tracing-subscriber**: For configuring how tracing events are processed and output (e.g., to console, file).
- **tracing-appender** (optional, for asynchronous file logging).

#### Interactions
Code emits tracing::info!, tracing::error!, tracing::debug! macros; tracing-subscriber formats and outputs these logs.

## Phase 5: Testing & Deployment Preparation

### 5.1 Unit Testing

#### Logic
Write isolated tests for individual functions and modules, mocking external dependencies where necessary (e.g., JWKS fetching, database calls for specific units).

#### Relationships
Test modules to application modules.

#### Entities
Mock objects, test data.

#### Libraries
- **tokio::test**: For writing asynchronous tests.
- **assert_eq!**, **assert_ne!**, **assert_matches!**, etc. (standard Rust assertions).
- **mockall** (optional, for mocking traits/structs).

#### Interactions
cargo test command execution.

### 5.2 Integration Testing

#### Logic
Write tests that exercise the interaction between multiple components (e.g., JWT middleware, role-based endpoint authorization, handlers, database). Use a test database for realistic scenarios. Test `echo` and `premium_echo` endpoints with valid/invalid tokens and roles.

#### Relationships
Test suite to running Axum application and test database.

#### Entities
Test database schema, test data (including users with different roles for JWT generation if mocking JWTs, or setup in a test Supabase instance if using live tokens).

#### Libraries
- **axum::test**: Utilities for sending HTTP requests to the Axum application in tests (or `reqwest` if testing an externalized service).
- **sqlx::test**: For managing test database transactions.
- **testcontainers** (optional, for spinning up temporary PostgreSQL containers).
- Potentially a helper library for generating test JWTs with specific claims/roles if not relying on an external Supabase instance for tokens.

#### Interactions
Test code sends HTTP requests to the test Axum server, asserts on responses (status codes, bodies) and database state.

### 5.3 Containerization

#### Logic
Create a Dockerfile to package the Rust Axum application into a lightweight, portable container image. Define environment variables for runtime configuration. Emphasize multi-stage builds for smaller production images.

#### Relationships
Rust project -> Docker image.

#### Entities
Dockerfile, .dockerignore.

#### Libraries
- **docker**: Containerization tool.

#### Interactions
docker build command to create the image; docker run to execute the container. 