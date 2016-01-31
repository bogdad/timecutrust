//  1   2   3  4  5 6 7 8 9 10
//  -4 -3  -2 -1  0 1 2 3 4 5
pub fn binary_search<'a, P>(pi_beg: u64, pi_end: u64, predicate: &'a mut P) -> u64
    where P : FnMut(u64) -> i64 {
    let mut i_beg = pi_beg;
    let mut i_end = pi_end;
    let pval = predicate(i_beg);
    if pval == 0 {
        return i_beg;
    }
    while i_beg <= i_end {
        let mid = i_beg + (i_end-i_beg)/2;
        let pval = predicate(mid);
        println!("{:?} {:?} {:?} {:?} \n", i_beg, i_end, mid, pval);
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


struct EqPred {
    item : i64
}

impl EqPred {
    fn new(item:i64) -> EqPred {
        EqPred{item: item}
    }
    fn call_inner(&self, pos:(u64,)) -> i64 {
        let (i,) = pos;
        let z:i64 = self.item.checked_add(i as i64).unwrap();
        z
    }
}

impl FnOnce<(u64,)> for EqPred {
    type Output = i64;
    extern "rust-call" fn call_once(self, pos: (u64,)) -> i64 {
        self.call_inner(pos)
    }
}

impl FnMut<(u64,)> for EqPred {
    extern "rust-call" fn call_mut(& mut self, pos: (u64,)) -> i64 {
        self.call_inner(pos)
    }
}


#[test]
fn test_binary_search() {
    let mut eq2 = EqPred::new(-2);
    let mut eq9 = EqPred::new(-9);
    let mut eq10 = EqPred::new(-10);
    let mut eq_1 = EqPred::new(1);
    assert_eq!(binary_search(0, 9, &mut eq2), 2);
    assert_eq!(binary_search(0, 9, &mut eq9), 9);
    assert_eq!(binary_search(0, 9, &mut eq10), 9);
    assert_eq!(binary_search(0, 9, &mut eq_1), 0);
}

