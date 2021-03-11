pub trait IteratorExtension {
    type Item : PartialEq;

    fn sequence_equals<T: IntoIterator<Item = Self::Item> + Clone>(&self, other: &T) -> bool;
}

impl<TIterator : IntoIterator + Clone> IteratorExtension for TIterator where TIterator::Item : PartialEq {
    type Item = TIterator::Item;

    fn sequence_equals<T: IntoIterator<Item = Self::Item> + Clone>(&self, other: &T) -> bool {
        self.clone().into_iter().zip(other.clone().into_iter()).all(|item| {item.0 == item.1})
    }
}