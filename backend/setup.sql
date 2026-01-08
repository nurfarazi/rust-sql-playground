-- Create Database
IF NOT EXISTS(SELECT * FROM sys.databases WHERE name = 'RustCrudApp')
BEGIN
    CREATE DATABASE RustCrudApp;
END
GO

USE RustCrudApp;
GO

-- Create Users Table
IF NOT EXISTS (SELECT * FROM sysobjects WHERE name='users' AND xtype='U')
BEGIN
    CREATE TABLE users (
        id INT IDENTITY(1,1) PRIMARY KEY,
        name NVARCHAR(100) NOT NULL,
        email NVARCHAR(100) NOT NULL,
        created_at DATETIME2 DEFAULT GETDATE()
    );
END
GO
