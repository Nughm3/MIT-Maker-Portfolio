use std::cell::OnceCell;

#[derive(Debug)]
pub struct SegmentList<T> {
    echelons: [OnceCell<Vec<T>>; 31],
    len: u32,
}

impl<T: Default> SegmentList<T> {
    const ONCE_CELL: OnceCell<Vec<T>> = OnceCell::new();

    pub fn new() -> Self {
        SegmentList {
            echelons: [Self::ONCE_CELL; 31],
            len: 0,
        }
    }

    pub fn get(&mut self, idx: u32) -> Option<&mut T> {
        let (echelon_idx, offset) = segment_index(idx);
        self.echelon(echelon_idx).get_mut(offset)
    }

    pub fn push(&mut self, item: T) -> u32 {
        let (echelon_idx, _) = segment_index(self.len);
        self.echelon(echelon_idx).push(item);

        self.len += 1;
        self.len - 1
    }

    fn echelon(&mut self, echelon_idx: usize) -> &mut Vec<T> {
        let cell = &mut self.echelons[echelon_idx as usize];

        if cell.get().is_none() {
            cell.set(Vec::with_capacity(1 << echelon_idx)).ok().unwrap();
        }

        cell.get_mut().unwrap()
    }
}

fn segment_index(idx: u32) -> (usize, usize) {
    let echelon_idx = prev_power_of_two(idx + 1).trailing_zeros() as usize;
    (echelon_idx, idx as usize - ((1 << echelon_idx) - 1))
}

fn prev_power_of_two(n: u32) -> u32 {
    if n.is_power_of_two() || n == 0 {
        n
    } else {
        n.next_power_of_two() >> 1
    }
}
