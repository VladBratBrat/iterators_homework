#![forbid(unsafe_code)]

pub struct FlatMap<InIter: Iterator, F: FnMut(InIter::Item) -> OutIter, OutIter: IntoIterator> {
    // TODO: your code here.
}

impl<InIter, F, OutIter> Iterator for FlatMap<InIter, F, OutIter> {
    // TODO: your code here.
}

pub fn flat_map<InputIterator, Mapping, OutputIterator>(
    iter: InputIterator,
    f: Mapping,
) -> FlatMap<InputIterator, Mapping, OutputIterator> {
    // TODO: your code here.
    unimplemented!()
}
