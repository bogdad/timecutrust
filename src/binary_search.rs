use std::cmp::max;

type Pred = (Box<Fn(i64) -> i64>);
pub type Predicate = Pred;

pub fn binary_search(beg: u64, end: u64, predicate: Predicate) -> i64 {
    binary_search_inner(predicate, beg, end)
}

//  1   2   3  4  5 6 7 8 9 10
//  -4 -3  -2 -1  0 1 2 3 4 5
fn binary_search_inner(
predicate: Predicate, pi_beg: u64, pi_end: u64) -> u64 {
    let mut i_beg = pi_beg;
    let mut i_end = pi_end;
    while i_beg <= i_end {
        let mid = i_beg + (i_end-i_beg)/2;
        let pval = predicate(mid);
        print!("{:?} {:?} {:?} {:?} \n", i_beg, i_end, mid, pval);
        if pval == 0 {
            return  mid;
        } else if pval < 0 {
            i_beg = mid + 1;
        } else {
            i_end = mid - 1;
        }
    };
    -(i_beg + 1)
}

fn predfactory (item:i64) -> Predicate {
    let pred: Pred =
    Box::new(move |i:i64| { let z:i64 = item.checked_add(i).unwrap(); z});
    pred
}


#[test]
fn test_binary_search() {
    let pred: Predicate = Box::new(|i| { let g:i64 = -5; let z:i64 = g.checked_add(i).unwrap(); z });
    let res = binary_search(0, 10, pred);
    assert_eq!(res, 5);
    assert_eq!(binary_search(0, 9, predfactory(-2)), 2);
    assert_eq!(binary_search(0, 9, predfactory(-9)), 9);
    assert_eq!(binary_search(0, 9, predfactory(-10)), -11);
    assert_eq!(binary_search(0, 9, predfactory(1)), -1);
}

