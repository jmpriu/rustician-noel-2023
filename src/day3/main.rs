use std::io::stdin;

fn is_number(c: char) -> bool {
    c.is_numeric()
}
fn is_empty(c: char) -> bool {
    c == '.'
}

fn is_symbol(c: char) -> bool {
    return !is_number(c) && !is_empty(c);
}
fn is_pos_near_symbol(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            let pos_x = x as i32 + i - 1;
            let pos_y: i32 = y as i32 + j - 1;
            if i == 1 && j == 1 { continue; }
            if pos_x < 0 || pos_y < 0 || pos_x >= grid.len() as i32|| pos_y >= grid[pos_x as usize].len() as i32 { continue; };
            if is_symbol(grid[pos_x as usize][pos_y as usize]) {
                return true;
            }
        }
    }
    return false;
}
fn main() {
    let stdin = stdin();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for line in stdin.lines().map(|l| l.unwrap()) {
        let mut row: Vec<char> = Vec::new();
        let mut visited_row: Vec<bool> = Vec::new();
        for char in line.chars() {
            row.push(char);
            visited_row.push(false);
        }
        grid.push(row);
        visited.push(visited_row);
    }
    let mut numbers_next_to_symbols: Vec<u32> = Vec::new();
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if is_number(grid[x][y]) && !visited[x][y] {
                let mut candidate = grid[x][y].to_digit(10).unwrap() as u32;
                let mut add_candidate = is_pos_near_symbol(&grid, x, y);
                for i in y + 1..grid[x].len() {
                    if is_number(grid[x][i]) {
                        visited[x][i] = true;
                        add_candidate = add_candidate | is_pos_near_symbol(&grid, x, i);
                        candidate = candidate * 10 + grid[x][i].to_digit(10).unwrap() as u32;
                    } else {
                        break;
                    }
                }
                if add_candidate {numbers_next_to_symbols.push(candidate); }
            }
        }
    }
    let sum = numbers_next_to_symbols.iter().sum::<u32>();
    println!("{}", sum);
}