use std::cmp::max;

type Pred = (Fn(u64) -> i64);
pub type Predicate = Pred;

pub fn binary_search<'a>(beg: u64, end: u64, predicate: &'a Predicate) -> u64 {
    print!("new case");
    binary_search_inner(predicate, beg, end)
}

//  1   2   3  4  5 6 7 8 9 10
//  -4 -3  -2 -1  0 1 2 3 4 5
fn binary_search_inner<'a>(predicate: &'a Predicate, pi_beg: u64, pi_end: u64) -> u64 {
    let mut i_beg = pi_beg;
    let mut i_end = pi_end;
    while i_beg <= i_end {
        let mid = i_beg + (i_end-i_beg)/2;
        let pval = predicate(mid);
        print!("{:?} {:?} {:?} {:?} \n", i_beg, i_end, mid, pval);
        if pval == 0 {
            return  mid;
        } else if pval < 0 {
            if mid < i_end {
                i_beg = mid + 1;
            } else {
                break;
            }
        } else {
            if mid > 0 {
                i_end = mid - 1;
            } else {
                break;
            }
        }
    };
    i_beg
}

fn predfactory<'a>(item:i64) -> &'a Predicate {
    & move |i:u64| { let z:i64 = item.checked_add(i as i64).unwrap(); z};
}


#[test]
fn test_binary_search() {
    let pred: Predicate = Box::new(|i:u64| { let g:i64 = -5; let z:i64 = g.checked_add(i as i64).unwrap(); z });
    let res = binary_search(0, 10, pred);
    assert_eq!(res, 5);
    assert_eq!(binary_search(0, 9, predfactory(-2)), 2);
    assert_eq!(binary_search(0, 9, predfactory(-9)), 9);
    assert_eq!(binary_search(0, 9, predfactory(-10)), 9);
    assert_eq!(binary_search(0, 9, predfactory(1)), 0);
}

