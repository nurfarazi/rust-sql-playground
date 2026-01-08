# Full Stack Rust & React User Management System

A robust, modern CRUD application demonstrating the integration of a high-performance **Rust** backend with a dynamic **React** frontend, persisted by **Microsoft SQL Server**.

![Project Status](https://img.shields.io/badge/status-active-success.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## 🏗 Architecture

| Component | Technology | Description |
| :--- | :--- | :--- |
| **Backend** | Rust, Axum, SQLx | High-speed, type-safe REST API using async Rust. |
| **Frontend** | React, TypeScript, Vite | Modern, fast SPA with strict type checking. |
| **Database** | SQL Server Express | Enterprise-grade RDBMS using Windows Authentication. |
| **Communication** | REST / JSON | Standard HTTP methods for data exchange. |

## 🚀 Prerequisites

Before you begin, ensure you have the following installed:

*   **Rust & Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)
*   **Node.js & npm**: [Install Node.js](https://nodejs.org/) (v18+ recommended)
*   **SQL Server Express**: [Download](https://www.microsoft.com/en-us/sql-server/sql-server-downloads) (Ensure "LocalDB" or "Express" instance is running).
    *   *Important*: Ensure TCP/IP is enabled in SQL Server Configuration Manager if not using LocalDB.

## 🛠️ Installation & Setup

### 1. Backend Setup

The backend handles API requests and database connections.

1.  **Navigate to the backend directory:**
    ```bash
    cd backend
    ```

2.  **Configure Environment:**
    Check the `.env` file. It defaults to a standard local SQL Express setup with Windows Authentication:
    ```env
    DATABASE_URL=sqlserver://localhost/SQLEXPRESS;database=RustCrudApp;integratedSecurity=true;TrustServerCertificate=true;
    ```
    *Modify `localhost/SQLEXPRESS` if your instance name differs.*

3.  **Run the Server:**
    ```bash
    cargo run
    ```
    *The server will compile (this may take a minute) and start on `http://localhost:3000`.*

### 2. Frontend Setup

The frontend provides the user interface.

1.  **Open a new terminal and navigate to the frontend directory:**
    ```bash
    cd frontend
    ```

2.  **Install Dependencies:**
    ```bash
    npm install
    ```

3.  **Start Development Server:**
    ```bash
    npm run dev
    ```
    *Access the app at `http://localhost:5173`.*

## 💾 Database Initialization

This project features an **Automatic / On-Demand Database Setup**. You do not need to manually run SQL scripts.

1.  Start both Backend and Frontend.
2.  Open the App in your browser (`http://localhost:5173`).
3.  If the database does not exist, you will see a connection error or empty list.
4.  Click the green **"Initialize Database"** button in the top right corner.
    *   This sends a command to the backend to create the `RustCrudApp` database and the `users` table.
5.  Once successful, the app is ready to use!

*(Optional) Manual Setup:*
If you prefer, you can execute the `backend/setup.sql` script using SSMS or `sqlcmd`.

## 📡 API Documentation

Base URL: `http://localhost:3000`

| Method | Endpoint | Description | Payload Example |
| :--- | :--- | :--- | :--- |
| `GET` | `/users` | Retrieve all users | - |
| `POST` | `/users` | Create a user | `{"name": "Alice", "email": "alice@example.com"}` |
| `PUT` | `/users/:id` | Update a user | `{"name": "Alice Cooper", "email": "alice@rocks.com"}` |
| `DELETE` | `/users/:id` | Delete a user | - |
| `POST` | `/init-db` | Init DB Schema | - |

## 🧩 Project Structure

```
rust-sql-playground/
├── backend/                # Rust API Server
│   ├── src/
│   │   ├── main.rs         # Entry point & App Config
│   │   ├── handlers.rs     # API Route Logic & DB Queries
│   │   └── models.rs       # Structs & Type Definitions
│   ├── Cargo.toml          # Rust Dependencies
│   └── setup.sql           # Manual SQL setup script
│
├── frontend/               # React Client
│   ├── src/
│   │   ├── App.tsx         # Main UI Component
│   │   └── App.css         # Styling
│   ├── package.json        # JS Dependencies
│   └── vite.config.ts      # Build Config
│
└── README.md               # You are here
```

## ❓ Troubleshooting

*   **"Failed to connect to database"**:
    *   Verify your SQL Server instance name. Is it `SQLEXPRESS` or `MSSQLSERVER`? Update `backend/.env`.
    *   Ensure **TCP/IP** is enabled in *SQL Server Configuration Manager* > *SQL Server Network Configuration* > *Protocols for SQLEXPRESS*.
    *   Restart the SQL Server service after changes.

*   **"TLS/SSL Error"**:
    *   Ensure `TrustServerCertificate=true` is in your connection string in `.env` (it is by default).

*   **CORS Errors**:
    *   The backend is configured to accept CORS from any origin for development (`CorsLayer::new().allow_origin(Any)`). Ensure the backend is running.