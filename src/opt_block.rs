use std::collections::HashMap;
use std::cmp::min;
use crate::opt_block;

fn init_block_count(block: &HashMap<String, bool>, count: &mut HashMap<String, i32>, reqs: &Vec<String>) {
    for req in reqs {
        let exist = block.get(req).unwrap();
        let mut dist = count.entry(req.clone()).or_insert(1_000_000_000);
        if *exist {
            *dist = 0;
        }
    }
}

fn update_block_from(block: &HashMap<String, bool>, prev_count: &HashMap<String, i32>, reqs: &Vec<String>) -> HashMap<String, i32> {
    let mut count = HashMap::with_capacity(prev_count.len());
    for req in reqs {
        if *block.get(req).unwrap() {
            count.insert(req.clone(), 0);
        } else {
            count.insert(req.clone(), prev_count.get(req).unwrap() + 1);
        }
    }
    count
}

fn reqwise_max_dist(min_left: &HashMap<String, i32>, min_right: &HashMap<String, i32>, reqs: &Vec<String>) -> i32 {
    let mut max_dist = -1;
    for req in reqs {
        let dist_to_req = min(min_left.get(req).unwrap(), min_right.get(req).unwrap());
        if max_dist < *dist_to_req {
            max_dist = *dist_to_req;
        }
    }
    max_dist
}

pub fn find_best_block(blocks: Vec<HashMap<String, bool>>, reqs: Vec<String>) -> usize {
    let mut init_left = HashMap::new();
    let mut init_right = HashMap::new();

    init_block_count(&blocks[0], &mut init_left, &reqs);
    init_block_count(&blocks[blocks.len()-1], &mut init_right, &reqs);

    let mut min_left = Vec::with_capacity(blocks.len());
    min_left.push(init_left);
    let mut min_right = Vec::with_capacity(blocks.len());
    min_right.push(init_right);

    for block in blocks[1..].iter() {
        min_left.push(update_block_from(block, &min_left[min_left.len()-1], &reqs))
    }

    for block in blocks[..blocks.len()-1].iter().rev() {
        min_right.push(update_block_from(block, &min_right[min_right.len()-1], &reqs))
    }
    min_right.reverse();

    println!("{:?}", min_left);
    println!("{:?}", min_right);
    
    let mut min_block_idx = 0;
    let mut best_block_dist = 1_000_000_000;
    for (idx, (block_min_left, block_min_right)) in min_left.iter().zip(min_right.iter()).enumerate() {
        let block_dist = reqwise_max_dist(block_min_left, block_min_right, &reqs);
        if block_dist < best_block_dist {
            best_block_dist = block_dist;
            min_block_idx = idx;
        }
    }

    min_block_idx
}

fn main() {
    let idx = find_best_block(
        vec![
            HashMap::from([
                ("gym".to_owned(), false), ("school".to_owned(), true), ("store".to_owned(), false)
            ]),
            HashMap::from([
                ("gym".to_owned(), true), ("school".to_owned(), false), ("store".to_owned(), false)
            ]),
            HashMap::from([
                ("gym".to_owned(), true), ("school".to_owned(), true), ("store".to_owned(), false)
            ]),
            HashMap::from([
                ("gym".to_owned(), false), ("school".to_owned(), true), ("store".to_owned(), false)
            ]),
            HashMap::from([
                ("gym".to_owned(), false), ("school".to_owned(), true), ("store".to_owned(), true)
            ])
        ],
        vec!["gym", "school", "store"].iter().map(|&s| s.to_owned()).collect(),
    );
    println!("best {}", idx);
}