# Last API

This project is a simple API built using Rust and the Axum framework. It provides endpoints for managing users and licenses, backed by a SQLite database.

## Project Structure

- `src/main.rs`: The entry point of the application. It sets up the Axum router and starts the server.
- `src/db.rs`: Contains the database initialization logic.
- `src/model.rs`: Defines the data models for the application.
- `src/repositories/`: Contains modules for interacting with the database.
    - `licenses_repository.rs`: Functions for inserting and querying licenses.
    - `users_repository.rs`: Functions for inserting, querying, updating, and deleting users.
- `src/routes/`: Contains modules for defining the API routes.
    - `licenses_routes.rs`: Defines the routes for license-related operations.
    - `users_routes.rs`: Defines the routes for user-related operations.
- `src/services/`: Contains modules for handling the business logic.
    - `licenses_service.rs`: Functions for creating licenses.
    - `users_service.rs`: Functions for creating, listing, updating, and deleting users.

## Endpoints

### Users

- `POST /users/`: Create a new user.
- `GET /users/list`: List all users.
- `DELETE /users/:id`: Delete a user by ID.
- `PATCH /users/:id`: Update a user by ID.

### Licenses

- `POST /licenses/`: Create a new license.

## Database

The application uses SQLite as the database. The database URL should be set in the `.env` file with the key `DATABASE_URL`.

## Running the Application

1. Ensure you have Rust and Cargo installed.
2. Set up the `.env` file with the `DATABASE_URL`.
3. Run the application using `cargo run`.

The server will start on `http://127.0.0.1:8080`.

## Dependencies

- `axum`: Web framework for building the API.
- `sqlx`: Library for interacting with the SQLite database.
- `dotenvy`: Library for loading environment variables from a `.env` file.
- `serde`: Library for serializing and deserializing data.

## License

This project is licensed under the MIT License.