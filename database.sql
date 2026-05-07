DROP DATABASE IF EXISTS tictactoe_db;
CREATE DATABASE tictactoe_db;
USE tictactoe_db;

CREATE TABLE partidas (
    id INT AUTO_INCREMENT PRIMARY KEY,
    jugador_x VARCHAR(50),
    jugador_o VARCHAR(50),
    ganador VARCHAR(50),
    fecha TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
