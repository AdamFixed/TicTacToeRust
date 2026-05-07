DROP DATABASE IF EXISTS tictactoe_db;
CREATE DATABASE tictactoe_db;
USE tictactoe_db;

CREATE TABLE matches (
    id INT AUTO_INCREMENT PRIMARY KEY,
    jugador_x VARCHAR(50),
    jugador_o VARCHAR(50),
    winner VARCHAR(50),
    loser VARCHAR(50),
    played_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
