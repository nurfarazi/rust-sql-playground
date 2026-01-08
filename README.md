# Rust + React SQL Server CRUD App

This is a basic CRUD application with a Rust (Axum + sqlx) backend and a React (Vite + TypeScript) frontend.
It uses SQL Server with Windows Authentication.

## Prerequisites

- Rust (cargo)
- Node.js (npm)
- SQL Server Express (or similar)

## Database Setup

1. Make sure your SQL Server is running.
2. Run the `backend/setup.sql` script in your SQL Server Management Studio (SSMS) or via `sqlcmd` to create the database and table.

   ```cmd
   sqlcmd -S .\SQLEXPRESS -E -i backend/setup.sql
   ```

   *Note: Adjust `.\SQLEXPRESS` if your instance name is different.*

3. Check `backend/.env` and ensure the connection string matches your setup.
   
   Default:
   ```
   DATABASE_URL=sqlserver://localhost/SQLEXPRESS;database=RustCrudApp;integratedSecurity=true;TrustServerCertificate=true;
   ```

## Running the Backend

1. Navigate to the `backend` folder.
2. Run the server:

   ```bash
   cd backend
   cargo run
   ```

   The server will start on `http://localhost:3000`.

## Running the Frontend

1. Navigate to the `frontend` folder.
2. Install dependencies (if not done):

   ```bash
   cd frontend
   npm install
   ```

3. Start the dev server:

   ```bash
   npm run dev
   ```

   The frontend will be available at `http://localhost:5173`.

## Usage

- Open the frontend URL.
- You can Add, Edit, and Delete users.
- Data is persisted in SQL Server.
