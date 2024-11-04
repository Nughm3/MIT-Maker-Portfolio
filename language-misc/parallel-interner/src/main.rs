use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, RandomState},
};

use interner::segment_list::SegmentList;

#[derive(Debug)]
struct ListTable<T, S = RandomState> {
    keys: HashMap<u64, u32>,
    hash_builder: S,

    value_count: u32,
    tags: SegmentList<ListTag>,
    data: SegmentList<Option<u32>>,

    cons_count: u32,
    cons: SegmentList<ListCons<T>>,
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
enum ListTag {
    Cons,
    #[default]
    Nil,
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct ListCons<T> {
    value: T,
    next: Option<u32>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum ListView<T> {
    Cons(T, Box<ListView<T>>),
    Nil,
}

impl<T> FromIterator<T> for ListView<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut list = ListView::Nil;

        while let Some(next) = iter.next() {
            list = ListView::Cons(next, Box::new(list));
        }

        list
    }
}

impl<T: Default + Clone + Hash, S: BuildHasher> ListTable<T, S> {
    fn intern(&mut self, list: &ListView<T>) -> u32 {
        let hash = self.hash_builder.hash_one(list);
        if let Some(key) = self.keys.get(&hash) {
            return *key;
        }

        let key = self.value_count + 1;
        self.keys.insert(hash, key);

        fn traverse_list<T: Default + Clone, S>(
            tbl: &mut ListTable<T, S>,
            list: &ListView<T>,
        ) -> Option<u32> {
            match list {
                ListView::Cons(value, next) => {
                    let tag_key = tbl.tags.push(ListTag::Cons);

                    let next = traverse_list(tbl, &next);

                    let value_key = tbl.cons.push(ListCons {
                        value: value.clone(),
                        next,
                    });
                    tbl.data.push(Some(value_key));

                    tbl.cons_count += 1;
                    tbl.value_count += 1;
                    Some(tag_key)
                }
                ListView::Nil => {
                    tbl.tags.push(ListTag::Nil);
                    tbl.data.push(None);
                    tbl.value_count += 1;
                    None
                }
            }
        }

        traverse_list(self, list);

        key
    }

    fn resolve(&mut self, mut key: u32) -> ListView<T> {
        let mut list = ListView::Nil;

        loop {
            match self.tags.get(key).unwrap() {
                ListTag::Cons => {
                    let idx = self.data.get(key).unwrap();
                    let ListCons { value, next } = self.cons.get(idx.unwrap()).unwrap();
                    list = ListView::Cons(value.clone(), Box::new(list));
                    if let Some(next) = next {
                        key = *next;
                    }
                }
                ListTag::Nil => break,
            }
        }

        list
    }
}

fn main() {
    let v: ListView<i32> = (1..=10).collect();
    let mut tbl = ListTable {
        keys: HashMap::new(),
        hash_builder: RandomState::new(),

        value_count: 0,
        tags: SegmentList::new(),
        data: SegmentList::new(),

        cons_count: 0,
        cons: SegmentList::new(),
    };

    let k = tbl.intern(&v); // (ech: 3, off: 4)
    dbg!(&tbl, k);
    assert_eq!(tbl.resolve(k), v);
}
