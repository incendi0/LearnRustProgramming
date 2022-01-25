use std::iter::FlatMap;

/// Rust’s for loop syntax is actually sugar for iterators.
/// for x in values // same as `values.into_iter()`
/// for x in &mut values { // same as `values.iter_mut()`
/// for x in &values { // same as `values.iter()`

/// 关联类型和泛型的区别，关联类型，从另一方面来说，仅允许 单个实现，因为一个类型仅能实现一个trait一次，这可以用来限制实现的数量。
/// 下面链接举例Add和Deref
/// https://stackoverflow.com/questions/32059370/when-is-it-appropriate-to-use-an-associated-type-versus-a-generic-type/32065644#32065644
pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.front_iter {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }
            if let Some(next_front_inner) = self.outer.next() {
                self.front_iter = Some(next_front_inner.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

/// DoubleEndedIterator: Iterator
/// 不用定义Item
impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.back_iter {
                if let Some(i) = inner_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }
            if let Some(next_back_inner) = self.outer.next_back() {
                self.back_iter = Some(next_back_inner.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

pub trait IteratorFlattenExt: Iterator {
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
        Self: Sized; // Flatten struct中有O作为field
}

impl<T> IteratorFlattenExt for T
where
    T: Iterator,
{
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
        Self: Sized,
    {
        flatten(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_works() {
        assert_eq!(flatten(std::iter::empty::<Vec<i32>>()).count(), 0);
    }

    #[test]
    fn empty_works_2() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
    }

    #[test]
    fn one_item_works() {
        assert_eq!(flatten(std::iter::once(vec![0])).count(), 1);
    }

    #[test]
    fn two_item_works() {
        assert_eq!(flatten(vec![vec![0], vec![1, 2]]).count(), 3);
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a", "b"], vec!["c", "d"]]);
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next_back(), Some("d"));
        assert_eq!(iter.next(), Some("b"));
        assert_eq!(iter.next(), Some("c"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn flatten_ext_works() {
        let xxs = vec![vec![0], vec![1, 2], vec![3, 4, 5]];
        assert_eq!(
            xxs.into_iter().our_flatten().collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5]
        );
    }
}
