use rand::Rng;

const HEIGHT:usize = 16;
const WIDTH:usize = 16;

fn print_board(board: [[u8; WIDTH]; HEIGHT]) -> i32 {
    println!("\x1B[2J\x1B[1;1H");
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let symbol;
            if board[i][j] == 0 { symbol = "⬛"}
            else { symbol = "⬜"}
            print!("{symbol} ");
        }
        print!("\n");
    }

    return 0;
}

fn update_board(board: [[u8; WIDTH]; HEIGHT]) -> [[u8; WIDTH]; HEIGHT] {
    let mut new_board = [[0u8; WIDTH]; HEIGHT];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let mut count = 0;
            
            let ri; // Ranges are inclusive..exclusive
            if i == 0 { ri = 0..2} else {ri = i-1..i+2}
            for row in ri {
                let cj;
                if j == 0 { cj = 0..2} else { cj = j-1..j+2}
                for col in cj {
                    if row < HEIGHT && col < WIDTH && !(row == i && col == j) {
                        if board[row][col] == 1 { count += 1; }
                    }
                }
            }

            if board[i][j] == 1 && (count == 2 || count == 3) { new_board[i][j] = 1}
            else if board[i][j] == 0 && count == 3 { new_board[i][j] = 1}
            else { new_board[i][j] = 0; }
        }
    };
    return new_board;
}

fn main() {
    let mut board = [[0u8; WIDTH]; HEIGHT];
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let num: u8 = rand::thread_rng().gen_range(0..2);
            board[i][j] = num;
        }
    }
    loop {
        print_board(board);
        let second = core::time::Duration::from_millis(1000);
        std::thread::sleep(second);
        board = update_board(board);
    }
}
