use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

impl Direction {
    fn turned_right(self) -> Self {
        DIRECTIONS[(self.to_index() + 1) % 4]
    }

    fn to_index(self) -> usize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }

    fn from_index(i: usize) -> Self {
        DIRECTIONS[i]
    }

    fn forward_offset(self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
        }
    }
}

struct MapState {
    rows: usize,
    cols: usize,
    map: Vec<Vec<char>>,
    guard_row: usize,
    guard_col: usize,
    direction: Direction,
}

fn state_id(r: usize, c: usize, d: Direction, cols: usize) -> usize {
    (r * cols + c) * 4 + d.to_index()
}

fn build_map(lines: &[&str]) -> MapState {
    let rows = lines.len();
    let cols = lines[0].len();
    let mut map = vec![vec!['.'; cols]; rows];
    let (mut gr, mut gc) = (0, 0);
    let mut dir = Direction::Up;

    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            match ch {
                '#' => map[r][c] = '#',
                '^' => {
                    gr = r;
                    gc = c;
                    dir = Direction::Up;
                }
                'v' => {
                    gr = r;
                    gc = c;
                    dir = Direction::Down;
                }
                '<' => {
                    gr = r;
                    gc = c;
                    dir = Direction::Left;
                }
                '>' => {
                    gr = r;
                    gc = c;
                    dir = Direction::Right;
                }
                _ => {}
            }
        }
    }
    map[gr][gc] = '.';
    MapState {
        rows,
        cols,
        map,
        guard_row: gr,
        guard_col: gc,
        direction: dir,
    }
}

fn build_graph(state: &MapState) -> (Vec<Option<usize>>, Vec<Vec<usize>>) {
    let rows = state.rows;
    let cols = state.cols;
    let map = &state.map;
    let total_states = rows * cols * 4;
    let mut next_state = vec![None; total_states];
    let mut reverse_edges = vec![Vec::new(); rows * cols];

    for r in 0..rows {
        for c in 0..cols {
            let cell_id = r * cols + c;
            for d_i in 0..4 {
                let d = Direction::from_index(d_i);
                let sid = cell_id * 4 + d_i;
                let (dr, dc) = d.forward_offset();
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                    next_state[sid] = None;
                } else {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if map[nr][nc] == '#' {
                        let nd = d.turned_right();
                        next_state[sid] = Some((r * cols + c) * 4 + nd.to_index());
                    } else {
                        let nid = (nr * cols + nc) * 4 + d.to_index();
                        next_state[sid] = Some(nid);
                        reverse_edges[nr * cols + nc].push(sid);
                    }
                }
            }
        }
    }
    (next_state, reverse_edges)
}

fn part_one(state: &MapState, next_state: &[Option<usize>]) -> usize {
    let cols = state.cols;
    let total_cells = state.rows * cols;
    let mut visited_cells = vec![false; total_cells];
    let mut visited_states = vec![false; total_cells * 4];
    let (mut r, mut c, mut d) = (state.guard_row, state.guard_col, state.direction);

    visited_cells[r * cols + c] = true;
    visited_states[state_id(r, c, d, cols)] = true;

    loop {
        let sid = state_id(r, c, d, cols);
        if let Some(nid) = next_state[sid] {
            if visited_states[nid] {
                break;
            }
            visited_states[nid] = true;
            let cell_id = nid / 4;
            visited_cells[cell_id] = true;
            r = cell_id / cols;
            c = cell_id % cols;
            d = Direction::from_index(nid % 4);
        } else {
            break;
        }
    }
    visited_cells.iter().filter(|&&v| v).count()
}

fn detect_cycle_from(start_id: usize, next_state: &[Option<usize>]) -> bool {
    let mut visited = vec![0u8; next_state.len()];
    let mut path = Vec::new();
    let mut current = start_id;

    loop {
        match visited[current] {
            0 => {
                visited[current] = 1;
                path.push(current);
                if let Some(nid) = next_state[current] {
                    current = nid;
                } else {
                    for &p in &path {
                        visited[p] = 2;
                    }
                    return false;
                }
            }
            1 => return true,
            _ => {
                for &p in &path {
                    visited[p] = 2;
                }
                return false;
            }
        }
    }
}

fn part_two(
    state: &MapState,
    next_state: &mut [Option<usize>],
    reverse_edges: &[Vec<usize>],
) -> usize {
    let start_id = state_id(
        state.guard_row,
        state.guard_col,
        state.direction,
        state.cols,
    );
    let start_pos = state.guard_row * state.cols + state.guard_col;
    let mut count = 0;

    for r in 0..state.rows {
        for c in 0..state.cols {
            let pos = r * state.cols + c;
            if pos == start_pos || state.map[r][c] != '.' || reverse_edges[pos].is_empty() {
                continue;
            }
            let mut changed = Vec::with_capacity(reverse_edges[pos].len());
            for &sid in &reverse_edges[pos] {
                let nd = Direction::from_index(sid % 4).turned_right();
                let nid = (sid / 4) * 4 + nd.to_index();
                let old = next_state[sid];
                if old != Some(nid) {
                    changed.push((sid, old));
                    next_state[sid] = Some(nid);
                }
            }
            if detect_cycle_from(start_id, next_state) {
                count += 1;
            }
            for (sid, old) in changed {
                next_state[sid] = old;
            }
        }
    }
    count
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let map_state = build_map(&lines);
    let (mut next_state, reverse_edges) = build_graph(&map_state);
    println!("part1: {}", part_one(&map_state, &next_state));
    println!(
        "part2: {}",
        part_two(&map_state, &mut next_state, &reverse_edges)
    );
}

