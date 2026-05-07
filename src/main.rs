slint::include_modules!();
use slint::Model;
use mysql::prelude::*;
use mysql::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;

    let url = "mysql://root:123@localhost:3306/tictactoe_db";
    let pool: Pool = Pool::new(url)?;
    let mut move_number = 1;

    // --- Start Game ---
    let ui_handle = ui.as_weak(); //
    ui.on_start_game(move || {
        let ui = ui_handle.unwrap();
        ui.set_game_started(true);
        ui.set_current_turn("X".into());
        ui.set_status_msg(format!("{}'s Turn (X) Number {}", ui.get_player_one_name(), move_number).into());
    });

    // --- Restart Game ---
    let ui_handle = ui.as_weak();
    ui.on_restart_game(move || {
        let ui = ui_handle.unwrap();
        let empty_board: Vec<slint::SharedString> = vec!["".into(); 9];
        ui.set_board(slint::ModelRc::new(slint::VecModel::from(empty_board)));
        ui.set_current_turn("X".into());
        ui.set_game_started(false);
        ui.set_player_one_name("".into());
        ui.set_player_two_name("".into());
        ui.set_status_msg("Enter names to start!".into());
        move_number = 1;
    });

    // --- Cell Clicked ---
    let ui_handle = ui.as_weak();
    let pool_clone = pool.clone();
    ui.on_cell_clicked(move |index: i32| {
         // Placeholder, you can implement move counting if needed
        let ui = ui_handle.unwrap();
        move_number += 1;
        // Don't allow moves if game is over (status contains "Winner" or "Draw")
        let status = ui.get_status_msg().to_string();
        if status.contains("Winner") || status.contains("Draw") {
            return;
        }

        let mut board: Vec<String> = ui.get_board().iter().map(|s: slint::SharedString| s.to_string()).collect();

        if board[index as usize] != "" { 
            return;
        }

        let symbol = ui.get_current_turn().to_string();
        board[index as usize] = symbol.clone();

        // Update UI
        let slint_board: Vec<slint::SharedString> = board.iter().map(|s| s.into()).collect();
        ui.set_board(slint::ModelRc::new(slint::VecModel::from(slint_board.clone()))); //Sobreescribir el tablero con el nuevo movimiento

        if let Some(winner_symbol) = check_winner(&board) {
            let p1 = ui.get_player_one_name().to_string();
            let p2 = ui.get_player_two_name().to_string();

            let (winner, loser) = if winner_symbol == "X" 
            {
                (p1, p2)
            } 
            else 
            {
                (p2, p1)
            };

            ui.set_status_msg(format!("Winner: {}! Press Reset to play again.", winner).into());

            // MySQL
            match pool_clone.get_conn() {
                Ok(mut conn) => {
                    let _ = conn.exec_drop(
                        "INSERT INTO matches (jugador_x, jugador_o, winner, loser, played_at) VALUES (?, ?, ?, ?, NOW())",
                        (&winner, &loser, &winner, &loser),
                    );
                }
                Err(e) => eprintln!("DB error: {}", e),
            }
        } else if board.iter().all(|c| !c.is_empty()) {
            // All cells filled, no winner = Draw
            ui.set_status_msg("It's a Draw! Press Reset to play again.".into());
        } else {
            // Switch turn
            let next_turn: &str = if symbol == "X" { "O" } else { "X" };
            ui.set_current_turn(next_turn.into());
            let next_name = if next_turn == "X" {
                ui.get_player_one_name()
            } else {
                ui.get_player_two_name()
            };
            ui.set_status_msg(format!("{}'s Turn ({}) Number {}", next_name, next_turn, move_number).into());
        }
    });

    ui.run()?;
    Ok(())
}

fn check_winner(board: &[String]) -> Option<String> {
    let lines = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // Cols
        [0, 4, 8], [2, 4, 6],             // Diagonals
    ]; // Dar todas las posibilidades de ganar para checkear si hay un ganador
    for line in lines {
        if !board[line[0]].is_empty()
            && board[line[0]] == board[line[1]]
            && board[line[1]] == board[line[2]]
        {
            return Some(board[line[0]].clone());
        }
    }
    None
}