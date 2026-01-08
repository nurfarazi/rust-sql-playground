# Rust User Management Backend

This is the backend service for the User CRUD application, built with **Rust**, **Axum**, and **SQLx**, targeting **Microsoft SQL Server**.

## Prerequisites

- **Rust**: Ensure you have Rust and Cargo installed. [Install Rust](https://www.rust-lang.org/tools/install)
- **SQL Server Express**: A local instance of SQL Server Express (or any MSSQL compatible server).

## Configuration

The application uses a `.env` file for configuration. A default one is created for you:

```env
DATABASE_URL=sqlserver://localhost/SQLEXPRESS;database=RustCrudApp;integratedSecurity=true;TrustServerCertificate=true;
```

*   **DATABASE_URL**: The connection string to your SQL Server instance.
    *   `integratedSecurity=true` uses Windows Authentication.
    *   `TrustServerCertificate=true` allows self-signed certificates (common in dev).

## Database Setup

You have two options to initialize the database:

### Option 1: Automatic Initialization (Recommended)
1.  Run the backend (see below).
2.  Use the "Initialize Database" button in the Frontend UI.
3.  *Or* send a POST request to: `http://localhost:3000/init-db`

### Option 2: Manual Setup
Run the `setup.sql` script provided in this directory against your SQL Server instance:

```cmd
sqlcmd -S .\SQLEXPRESS -E -i setup.sql
```

## Running the Application

1.  Navigate to the `backend` directory:
    ```bash
    cd backend
    ```
2.  Run the server:
    ```bash
    cargo run
    ```

The server will start listening on `http://127.0.0.1:3000`.

## API Endpoints

| Method | Endpoint      | Description             | Body (JSON)             |
| :----- | :------------ | :---------------------- | :---------------------- |
| GET    | `/users`      | List all users          | -                       |
| POST   | `/users`      | Create a new user       | `{"name": "...", "email": "..."}` |
| PUT    | `/users/:id`  | Update an existing user | `{"name": "...", "email": "..."}` |
| DELETE | `/users/:id`  | Delete a user           | -                       |
| POST   | `/init-db`    | Initialize DB & Table   | -                       |
