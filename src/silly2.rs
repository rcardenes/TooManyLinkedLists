pub struct List<'a, T> {
    pub data: T,
    pub prev: Option<&'a List<'a, T>>,
}

pub struct Iter<'a, T> {
    next: Option<&'a List<'a, T>>,
}

impl <'a, T> List<'a, T> {
    pub fn push<U>(
        prev: Option<&'a List<'a, T>>,
        data: T,
        callback: impl FnOnce(&List<'a, T>) -> U
    ) -> U {
        let list = List { data, prev };
        callback(&list)
    }

    pub fn iter(&'a self) -> Iter<'a, T>{
        Iter { next: Some(self) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.prev;
            &node.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn elegance() {
        use std::cell::Cell;

        List::push(None, Cell::new(3), |list| {
            List::push(Some(list), Cell::new(5), |list| {
                List::push(Some(list), Cell::new(13), |list| {
                    for val in list.iter() {
                        val.set(val.get() * 10);
                    }

                    let mut vals = list.iter();
                    assert_eq!(vals.next().unwrap().get(), 130);
                    assert_eq!(vals.next().unwrap().get(), 50);
                    assert_eq!(vals.next().unwrap().get() ,30);
                    assert_eq!(vals.next(), None);
                    assert_eq!(vals.next(), None);
                })
            })
        })
    }
}