fn main() {
    println!("{}", solve(&mut vec![1, 3, 5], &3, &10));
}

fn solve(k: &mut Vec<i32>, n: &i32, m: &i32) -> String {
    k.sort();
    let mut f = false;

    for a in 0..*n {
        for b in 0..*n {
            for c in 0..*n {
                let target = m - k[a as usize] - k[b as usize] - k[c as usize];
                if binary_search(&k, &target){
                    f = true;
                }
            }
        }
    }

    if f { "Yes".to_string() } else { "No".to_string() }
}

fn binary_search(arr: &Vec<i32>, target: &i32) -> bool {
    let res = arr.binary_search(&target);
    match res {
        Ok(_) => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        assert_eq!(solve(&mut vec![1, 3, 5], &3, &10), "Yes");
        assert_eq!(solve(&mut vec![1, 3, 5], &3, &9), "No");
    }
}