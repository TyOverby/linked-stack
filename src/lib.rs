#[derive(Clone, Eq, PartialEq)]
pub struct LinkedStack<T, K> {
    tag: K,
    current: Vec<T>,
    previous: Option<Box<LinkedStack<T, K>>>,
}

impl <T, K> LinkedStack<T, K> {
    pub fn new(tag: K) -> LinkedStack<T, K> {
        LinkedStack {
            tag: tag,
            current: vec![],
            previous: None
        }
    }

    pub fn len(&self) -> usize {
        self.current.len() + self.previous.as_ref().map(|n| n.len()).unwrap_or(0)
    }

    pub fn push(&mut self, t: T) {
        self.current.push(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        let r = self.current.pop();
        if r.is_some() { return r; }

        if self.previous.is_none() { return None; }

        *self = *self.previous.take().unwrap();
        self.pop()
    }

    pub fn start_segment(&mut self, tag: K) {
        use std::mem::swap;
        let mut new = LinkedStack::new(tag);
        swap(self, &mut new);
        self.previous = Some(Box::new(new));
    }

    pub fn split(&mut self, tag: K) -> Option<LinkedStack<T, K>> where K: Eq {
        fn split_impl<T, K>(location: &mut LinkedStack<T, K>, tag: K) -> Option<Box<LinkedStack<T, K>>> where K: Eq {
            if location.tag == tag {
                location.previous.take()
            } else if location.previous.is_some() {
                split_impl(location.previous.as_mut().unwrap(), tag)
            } else {
                None
            }
        }

        split_impl(self, tag).map(|a| *a)
    }

    pub fn connect(&mut self, mut additional: LinkedStack<T, K>) {
        use std::mem::swap;
        swap(self, &mut additional);
        let original = additional;

        fn connect_impl<T, K>(target: &mut LinkedStack<T, K>, original: Box<LinkedStack<T, K>>) {
            if target.previous.is_none() {
                target.previous = Some(original);
            } else {
                connect_impl(target.previous.as_mut().unwrap(), original);
            }
        }

        connect_impl(self, Box::new(original))
    }
}
