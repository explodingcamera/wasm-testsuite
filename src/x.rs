use std::{
    iter::{FlatMap, Map, Take},
    mem::MaybeUninit,
    slice,
};

type Node<T, const N: usize> = [MaybeUninit<T>; N];
type NodeIter<'a, T, const N: usize> = slice::Iter<'a, Node<T, N>>;
type List<T, const N: usize> = [Node<T, N>];

pub struct ListIter<'a, T: Sized, const N: usize> {
    inner: FlatMap<
        NodeIter<'a, T, N>,
        Map<Take<slice::Iter<'a, MaybeUninit<T>>>, fn(&MaybeUninit<T>) -> T>,
        fn(&'a Node<T, N>) -> Map<Take<slice::Iter<'a, MaybeUninit<T>>>, fn(&MaybeUninit<T>) -> T>,
    >,
    _marker: core::marker::PhantomData<&'a List<T, N>>,
}
