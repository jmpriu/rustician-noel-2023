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
fn is_gear(c: char) -> bool {
    c == '*'
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
fn part1(grid: &Vec<Vec<char>>) {
    let mut numbers_next_to_symbols: Vec<u32> = Vec::new();
    let mut visited = generate_empty_flag_grid(&grid);
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

fn generate_empty_flag_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for x in 0..grid.len() {
        let mut row: Vec<bool> = Vec::new();
        for _ in 0..grid[x].len() {
            row.push(false);
        }
        visited.push(row);
    }
    return visited;
}

fn find_numbers_gear(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut visited = generate_empty_flag_grid(&grid);
    for i in 0..3 {
        for j in 0..3 {
            let pos_x = x as i32 + i - 1;
            let pos_y: i32 = y as i32 + j - 1;
            if i == 1 && j == 1 { continue; }
            if pos_x < 0 || pos_y < 0 || pos_x >= grid.len() as i32|| pos_y >= grid[pos_x as usize].len() as i32 { continue; };
            if is_number(grid[pos_x as usize][pos_y as usize]) && !visited[pos_x as usize][pos_y as usize] {
                visited[pos_x as usize][pos_y as usize] = true;
                let mut start = pos_y;
                loop {
                    visited[pos_x as usize][start as usize] = true;
                    if start < 1 || !is_number(grid[pos_x as usize][start as usize - 1]) {
                        break;
                    }
                    start -= 1;
                }
                let mut end = pos_y + 1;
                while end < grid[pos_x as usize].len() as i32 && is_number(grid[pos_x as usize][end as usize]) {
                    visited[pos_x as usize][end as usize] = true;
                    end += 1;
                }
                let mut number = grid[pos_x as usize][start as usize].to_digit(10).unwrap() as u32;
                if start != end {
                    for x in start + 1..end {
                        number = number * 10 + grid[pos_x as usize][x as usize].to_digit(10).unwrap() as u32;
                    }
                }
                numbers.push(number);
            }
        }
    }
    return numbers;
}
fn part2(grid: &Vec<Vec<char>>) {
    let mut result = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if is_gear(grid[x][y]) {
                let numbers = find_numbers_gear(&grid, x, y);
                if numbers.len() == 2 {
                    result += numbers[0] * numbers[1];
                }
            }
        }
    }
    println!("{}", result);
}
fn main() {
    let stdin = stdin();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in stdin.lines().map(|l| l.unwrap()) {
        let mut row: Vec<char> = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        grid.push(row);
    }
    println!("Part 1:");
    part1(&grid);
    println!("Part 2");
    part2(&grid);
}