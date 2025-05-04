use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct GameData {
    game: Vec<Game>,
}

#[derive(Debug, Deserialize)]
struct Game {
    p00: [i32; 2],
    p01: [i32; 2],
    p10: [i32; 2],
    p11: [i32; 2],
}
#[derive(Debug, Deserialize)]
struct FindDSE {
    a1: bool,
    b1: bool,
    a2: bool,
    b2: bool,
}
#[derive(Debug, Clone, Copy)]
struct Pref(i32, i32); //preference

type Matrix = [[Pref; 2]; 2];

fn build_matrix(game: &Game) -> Matrix {
    [
        [
            Pref(game.p00[0], game.p00[1]),
            Pref(game.p01[0], game.p01[1]),
        ],
        [
            Pref(game.p10[0], game.p10[1]),
            Pref(game.p11[0], game.p11[1]),
        ],
    ]
}

fn is_nash_equilibrium(matrix: &Matrix, row: usize, col: usize) -> bool {
    let current = matrix[row][col];
    let player1_best = (0..2).all(|r| matrix[r][col].0 <= current.0);
    let player2_best = (0..2).all(|c| matrix[row][c].1 <= current.1);
    player1_best && player2_best
}

fn is_pareto_dominated(a: &Pref, b: &Pref) -> bool {
    //2点を二次元平面上にプロットしたとき、片方がもう片方の右上にあるか判定
    //厳しいパレート最適性の定義なので境界は含まない
    b.0 > a.0 && b.1 > a.1
}

fn find_pareto_efficience(matrix: &Matrix) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for i in 0..2 {
        for j in 0..2 {
            let current = matrix[i][j];
            let dominated = matrix
                .iter()
                .flatten()
                .any(|&other| is_pareto_dominated(&current, &other));
            if !dominated {
                //1つでも右上にPref(選好の組)の点があるならその点はパレート最適でない(このときdominated = true)
                result.push((i, j));
            }
        }
    }
    result
}

fn find_dominant_strategy(matrix: &Matrix) {
    //最適応答の探索
    let mut find_dse = FindDSE {
        a1: false,
        b1: false,
        a2: false,
        b2: false,
    };
    if matrix[0][0].0 > matrix[1][0].0 && matrix[0][1].0 > matrix[1][1].0 {
        find_dse.a1 = true;
        // println!("  主体1の戦略a_1が支配戦略");
    } else if matrix[1][0].0 > matrix[0][0].0 && matrix[1][1].0 > matrix[0][1].0 {
        find_dse.b1 = true;
        // println!("  主体1の戦略b_1が支配戦略");
    }

    if matrix[0][0].1 > matrix[0][1].1 && matrix[1][0].1 > matrix[1][1].1 {
        find_dse.a2 = true;
        // println!("  主体2の戦略a_2が支配戦略");
    } else if matrix[0][1].1 > matrix[0][0].1 && matrix[1][1].1 > matrix[1][0].1 {
        find_dse.b2 = true;
        // println!("  主体2の戦略b_2が支配戦略");
    }

    let strategies = [
        ("a1", find_dse.a1),
        ("b1", find_dse.b1),
        ("a2", find_dse.a2),
        ("b2", find_dse.b2),
    ];
    let mut find_dse = false;
    for i in 0..strategies.len() {
        for j in i + 1..strategies.len() {
            if strategies[i].1 && strategies[j].1 {
                println!("  DSE: ({}, {})", strategies[i].0, strategies[j].0);
                find_dse = true;
            }
        }
    }
    if !find_dse {
        println!("  DSE: None");
    }
}

fn main() {
    let content = fs::read_to_string("game.toml").expect("ファイル読み込み失敗");
    let data: GameData = toml::from_str(&content).expect("TOML解析失敗");
    let result = [[("a1", "a2"), ("a1", "b2")], [("b1", "a2"), ("b1", "b2")]];

    for (i, game) in data.game.iter().enumerate() {
        match i {
            8 => println!("--- Game {} (The Gift of the Magi) ---", i + 1),
            11 => println!("--- Game {} (Prisoners’ dilemma) ---", i + 1),
            65 => println!("--- Game {} (Chicken game) ---", i + 1),
            68 => println!("--- Game {} (Battle of the sexes) ---", i + 1),
            _ => println!("--- Game {} ---", i + 1),
        }
        let matrix = build_matrix(game);

        find_dominant_strategy(&matrix);
        let mut nash_outcome_labels = Vec::new();
        for k in 0..2 {
            for j in 0..2 {
                if is_nash_equilibrium(&matrix, k, j) {
                    let (s1, s2) = result[k][j];
                    nash_outcome_labels.push(format!("({}, {})", s1, s2));
                }
            }
        }
        if nash_outcome_labels.is_empty() {
            println!("  Nash equilibria: None");
        } else {
            println!("  Nash equilibria: {}", nash_outcome_labels.join(", "));
        }

        let pareto = find_pareto_efficience(&matrix);
        let mut outcome_labels = Vec::new();
        for (k, j) in pareto {
            let (s1, s2) = result[k][j];
            outcome_labels.push(format!("({}, {})", s1, s2));
        }
        if outcome_labels.is_empty() {
            println!("  Pareto efficient outcomes: None");
        } else {
            println!("  Pareto efficient outcomes: {}", outcome_labels.join(", "));
        }
        println!();
    }
}
