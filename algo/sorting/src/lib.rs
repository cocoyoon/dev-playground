pub enum SortingAlgo {
    Selection,
    Insertion,
    ThreeNumberSort,
    Merge,
    Quick,
    Heap,
    Radix
}

impl SortingAlgo {
    pub fn sort<T: Ord>(with: Self, arr: &mut Vec<T>) {
        match with {
            Self::Selection => selection_sort(arr),
            _ => return
        }
    }
}

pub fn selection_sort<T: Ord> (arr: &mut Vec<T>) {

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_works() {
        let mut test_vec = vec![6,5,4,3,2,1];
        SortingAlgo::sort(SortingAlgo::Selection, &mut test_vec);
        assert_eq!(
            test_vec,
            [1,2,3,4,5,6].to_vec()
        )
    }
}