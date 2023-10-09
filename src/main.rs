use minheap::minheap::Heap;
use rand::seq::SliceRandom;
use std::collections::HashMap;

mod minheap;
fn main() {
    let start = shuffled_board();
    let goal = &new_board();
    println!("{:?}", start.state);
    let solution = a_star(start, goal);
    println!("{}", solution.len());
    for step in solution {
        for row in step.state {
            println!("{:?}", row);
        }
        println!();
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Board {
    state: [[usize; 4]; 4],
}

fn new_board() -> Board {
    Board {
        state: [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 0]],
    }
}

fn heuristic(board: &Board, goal_tuple: &[[usize; 2]; 16]) -> i32 {
    let mut sum = 0;
    for i in 0..4 {
        for j in 0..4 {
            if board.state[i][j] != 0 {
                sum += goal_tuple[board.state[i][j]][0].abs_diff(i);
                sum += goal_tuple[board.state[i][j]][1].abs_diff(j);
            }
        }
    }
    sum as i32
}

fn gen_goal_tuple(goal: &Board) -> [[usize; 2]; 16] {
    let mut tuple: [[usize; 2]; 16] = [[0; 2]; 16];
    for i in 0..4 {
        for j in 0..4 {
            tuple[goal.state[i][j]][0] = i;
            tuple[goal.state[i][j]][1] = j;
        }
    }
    tuple
}

fn shuffled_board() -> Board {
    let mut board = new_board();
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        board = *board.possible_moves().choose(&mut rng).unwrap();
    }
    board
}

impl Board {
    pub fn possible_moves(&self) -> Vec<Board> {
        let hole_index = self.hole_index();
        let mut possibilites = Vec::new();
        let mut copy: Board;
        if hole_index.0 >= 1 {
            copy = *self;
            copy.state[hole_index.0][hole_index.1] = copy.state[hole_index.0 - 1][hole_index.1];
            copy.state[hole_index.0 - 1][hole_index.1] = 0;
            possibilites.push(copy);
        }
        if hole_index.0 <= 2 {
            copy = *self;
            copy.state[hole_index.0][hole_index.1] = copy.state[hole_index.0 + 1][hole_index.1];
            copy.state[hole_index.0 + 1][hole_index.1] = 0;
            possibilites.push(copy);
        }
        if hole_index.1 >= 1 {
            copy = *self;
            copy.state[hole_index.0][hole_index.1] = copy.state[hole_index.0][hole_index.1 - 1];
            copy.state[hole_index.0][hole_index.1 - 1] = 0;
            possibilites.push(copy);
        }
        if hole_index.1 <= 2 {
            copy = *self;
            copy.state[hole_index.0][hole_index.1] = copy.state[hole_index.0][hole_index.1 + 1];
            copy.state[hole_index.0][hole_index.1 + 1] = 0;
            possibilites.push(copy);
        }
        possibilites
    }

    fn hole_index(&self) -> (usize, usize) {
        for i in 0..4 {
            for j in 0..4 {
                if self.state[i][j] == 0 {
                    return (i, j);
                }
            }
        }
        unreachable!();
    }
}

fn reconstruct_path(
    current_node_index: usize,
    node_table: Vec<(Board, i32, i32, usize)>,
) -> Vec<Board> {
    let mut index = current_node_index;
    let mut path: Vec<Board> = Vec::new();
    while index != 0 {
        path.push(node_table[index].0);
        index = node_table[index].3;
    }
    path.push(node_table[0].0);
    path.into_iter().rev().collect()
}

fn a_star(start: Board, goal: &Board) -> Vec<Board> {
    // the cheapest path from start to that node + heuristic cost for the node, with a usize that points to the nodes spot in the node table
    let mut prior_queue = minheap::minheap::initialize();
    // of the form (node, cheapest path, heuristic, pointer to prior node)
    let mut node_table: Vec<(Board, i32, i32, usize)> = Vec::new();

    let mut double_map: HashMap<Board, usize> = HashMap::new();

    let goal_tuple = gen_goal_tuple(goal);

    let mut current_node: &Board;
    let mut current_node_index: (i32, usize);
    let mut tentative_path_cost: i32;

    prior_queue.push(0, 0);
    node_table.push((start, 0, 0, 0));
    double_map.insert(start, 0);

    while !prior_queue.is_empty() {
        current_node_index = prior_queue.pop();
        current_node = &node_table[current_node_index.1].0;

        if current_node == goal {
            return reconstruct_path(current_node_index.1, node_table);
        }

        for child in current_node.possible_moves() {
            tentative_path_cost = node_table[current_node_index.1].1 + 1;
            let heuristic_value = heuristic(&child, &goal_tuple);
            let pos_in = double_map.get(&child);
            // ah fuck this does need to be done with hashmaps huh
            if pos_in != None {
                if tentative_path_cost < node_table[*pos_in.unwrap()].1 {
                    node_table[*pos_in.unwrap()] = (
                        child,
                        tentative_path_cost,
                        heuristic_value + tentative_path_cost,
                        current_node_index.1,
                    );
                }
            } else {
                double_map.insert(child, node_table.len());
                node_table.push((
                    child,
                    tentative_path_cost,
                    heuristic_value + tentative_path_cost,
                    current_node_index.1,
                ));
                prior_queue.push(node_table[node_table.len() - 1].2, node_table.len() - 1);
            }
        }
    }

    Vec::new()
}
