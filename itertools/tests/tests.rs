use std::{cell::Cell, rc::Rc};

struct Clonable {
    payload: usize,
    cloned_count: Rc<Cell<usize>>,
}

impl Clonable {
    fn new(payload: usize) -> Self {
        Self {
            payload,
            cloned_count: Rc::new(Cell::new(0)),
        }
    }

    fn cloned_count(&self) -> usize {
        self.cloned_count.get()
    }
}

impl Clone for Clonable {
    fn clone(&self) -> Self {
        self.cloned_count.set(self.cloned_count.get() + 1);
        Self {
            payload: self.payload,
            cloned_count: self.cloned_count.clone(),
        }
    }
}

struct TrackedIterator<I> {
    inner: I,
    advanced_count: Rc<Cell<usize>>,
}

impl<I> TrackedIterator<I> {
    fn new<T: IntoIterator<IntoIter = I>>(into_iter: T) -> (Self, Rc<Cell<usize>>) {
        let advanced_count = Rc::new(Cell::new(0));
        let iter = Self {
            inner: into_iter.into_iter(),
            advanced_count: advanced_count.clone(),
        };
        (iter, advanced_count)
    }
}

impl<I: Iterator> Iterator for TrackedIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.advanced_count.set(self.advanced_count.get() + 1);
        self.inner.next()
    }
}

struct Iterable<I> {
    inner: I,
}

impl<I: Iterator> IntoIterator for Iterable<I> {
    type Item = I::Item;
    type IntoIter = I;

    fn into_iter(self) -> Self::IntoIter {
        self.inner
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn count_lines_with_word() {
    let lines1 = vec![
        "Hello world".to_string(),
        "This is a test".to_string(),
        "World peace".to_string(),
        "Another line".to_string(),
    ];
    assert_eq!(
        itertools::count_lines_with_word(lines1.into_iter(), "world"),
        2
    );

    let lines2 = vec![
        "HELLO WORLD".to_string(),
        "hello world".to_string(),
        "Hello World".to_string(),
        "No match here".to_string(),
    ];
    assert_eq!(
        itertools::count_lines_with_word(lines2.into_iter(), "hello"),
        3
    );

    let lines3 = vec![
        "This is a test".to_string(),
        "Testing is important".to_string(),
        "No match".to_string(),
        "Attest to this".to_string(),
    ];
    assert_eq!(
        itertools::count_lines_with_word(lines3.into_iter(), "test"),
        3
    );

    let empty_lines = vec![];
    assert_eq!(
        itertools::count_lines_with_word(empty_lines.into_iter(), "word"),
        0
    );

    let lines5 = vec![
        "Apple banana".to_string(),
        "Cherry date".to_string(),
        "Fig grape".to_string(),
    ];
    assert_eq!(
        itertools::count_lines_with_word(lines5.into_iter(), "orange"),
        0
    );

    let lines6 = vec![
        "word at start".to_string(),
        "middle word here".to_string(),
        "ends with word".to_string(),
        "no match".to_string(),
    ];
    assert_eq!(
        itertools::count_lines_with_word(lines6.into_iter(), "word"),
        3
    );

    let lines7 = vec![
        "word word word".to_string(),
        "another line".to_string(),
        "word again word".to_string(),
    ];
    assert_eq!(
        itertools::count_lines_with_word(lines7.into_iter(), "word"),
        2
    );
}

#[test]
fn test_words_counter_iter() {
    let lines1 = vec![
        "hello world hello".to_string(),
        "this is a test".to_string(),
        "hello hello hello".to_string(),
        "world".to_string(),
    ];
    let result1 = itertools::words_counter_iter(lines1.into_iter(), "hello").collect::<Vec<_>>();
    assert_eq!(result1, vec![(2, 0), (3, 2)]);

    let lines2 = vec![
        "HELLO world".to_string(),
        "Hello World".to_string(),
        "hello hello".to_string(),
        "no match".to_string(),
    ];
    let result2 = itertools::words_counter_iter(lines2.into_iter(), "hello").collect::<Vec<_>>();
    assert_eq!(result2, vec![(1, 0), (1, 1), (2, 2)]);

    let empty_lines = vec![];
    let result3 =
        itertools::words_counter_iter(empty_lines.into_iter(), "word").collect::<Vec<_>>();
    assert_eq!(result3, vec![]);

    let lines4 = vec![
        "apple banana".to_string(),
        "cherry date".to_string(),
        "fig grape".to_string(),
    ];
    let result4 = itertools::words_counter_iter(lines4.into_iter(), "orange").collect::<Vec<_>>();
    assert_eq!(result4, vec![]);

    let lines5 = vec![
        "testing is important".to_string(),
        "attest to this".to_string(),
        "no test here".to_string(),
        "bestest".to_string(),
    ];
    let result5 = itertools::words_counter_iter(lines5.into_iter(), "test").collect::<Vec<_>>();
    assert_eq!(result5, vec![(1, 0), (1, 1), (1, 2), (1, 3)]);

    let lines6 = vec![
        "first line".to_string(),
        "second word line".to_string(),
        "third line".to_string(),
        "fourth word word".to_string(),
    ];
    let result6 = itertools::words_counter_iter(lines6.into_iter(), "word").collect::<Vec<_>>();
    assert_eq!(result6, vec![(1, 1), (2, 3)]);

    let lines7 = vec![
        "word word word".to_string(),
        "single word".to_string(),
        "word".to_string(),
    ];
    let result7 = itertools::words_counter_iter(lines7.into_iter(), "word").collect::<Vec<_>>();
    assert_eq!(result7, vec![(3, 0), (1, 1), (1, 2)]);

    let lines8 = vec!["aaa".to_string()];
    let result8 = itertools::words_counter_iter(lines8.into_iter(), "a").collect::<Vec<_>>();
    assert_eq!(result8, vec![(3, 0)]);
}

#[test]
fn count() {
    let mut iter = itertools::count();
    for i in 0..100 {
        assert_eq!(iter.next(), Some(i));
    }

    assert_eq!(iter.skip(3000).next(), Some(3100));
}

#[test]
fn cycle_simple() {
    let elems = vec![Clonable::new(0), Clonable::new(1)];
    let (tracked_iter, advanced_count) = TrackedIterator::new(elems.clone());
    let iterable = Iterable {
        inner: tracked_iter,
    };

    let mut iter = itertools::cycle(iterable);
    assert_eq!(advanced_count.get(), 0);
    assert_eq!(elems[0].cloned_count(), 1);
    assert_eq!(elems[1].cloned_count(), 1);

    let first = iter.next().unwrap();
    assert_eq!(advanced_count.get(), 1);
    assert_eq!(first.payload, 0);
    assert_eq!(elems[0].cloned_count(), 2);
    assert_eq!(elems[1].cloned_count(), 1);

    let second = iter.next().unwrap();
    assert_eq!(advanced_count.get(), 2);
    assert_eq!(second.payload, 1);
    assert_eq!(elems[0].cloned_count(), 2);
    assert_eq!(elems[1].cloned_count(), 2);

    let third = iter.next().unwrap();
    assert_eq!(advanced_count.get(), 3);
    assert_eq!(third.payload, 0);
    assert_eq!(elems[0].cloned_count(), 3);
    assert_eq!(elems[1].cloned_count(), 2);

    for i in 0..1000 {
        let elem = iter.next().unwrap();
        assert_eq!(advanced_count.get(), 3);
        assert_eq!(elem.payload, (i + 1) % 2);
        assert_eq!(elems[0].cloned_count(), 3 + (i + 1) / 2);
        assert_eq!(elems[1].cloned_count(), 3 + i / 2);
    }
}

#[test]
fn cycle_empty() {
    let mut iter = itertools::cycle(Vec::<i32>::new());
    assert!(iter.next().is_none());
}

#[test]
fn extract_simple() {
    struct Int(usize);
    let mut iter: Box<dyn Iterator<Item = Int>> = Box::new((0..100).map(Int));

    for i in (1..100).step_by(2) {
        let (mb_extracted, new_iter) = itertools::extract(iter, 1 + i / 2);
        assert_eq!(mb_extracted.unwrap().0, i);
        iter = Box::new(new_iter);
    }

    assert_eq!(
        iter.map(|x| x.0).collect::<Vec<_>>(),
        (0..100).step_by(2).collect::<Vec<_>>()
    );
}

#[test]
fn extract_nonexistent() {
    let (mb_extracted, iter) = itertools::extract(vec![0, 1, 2], 10);
    assert!(mb_extracted.is_none());
    assert_eq!(iter.collect::<Vec<_>>(), vec![0, 1, 2]);
}

#[test]
fn tee_simple() {
    let elems = (0..5).map(Clonable::new).collect::<Vec<_>>();
    let (tracked_iter, advanced_count) = TrackedIterator::new(elems.clone());
    let iterable = Iterable {
        inner: tracked_iter,
    };

    let (mut first, mut second) = itertools::tee(iterable);
    assert_eq!(advanced_count.get(), 0);
    assert!(elems.iter().all(|e| e.cloned_count() == 1));

    for i in 0..2 {
        assert_eq!(first.next().unwrap().payload, i);
        assert_eq!(advanced_count.get(), i + 1);
        assert!(elems[..i + 1].iter().all(|e| e.cloned_count() == 2));
        assert!(elems[i + 1..].iter().all(|e| e.cloned_count() == 1));
    }

    for i in 0..2 {
        assert_eq!(second.next().unwrap().payload, i);
        assert_eq!(advanced_count.get(), 2);
        assert!(elems[..2].iter().all(|e| e.cloned_count() == 2));
        assert!(elems[2..].iter().all(|e| e.cloned_count() == 1));
    }

    for i in 2..5 {
        assert_eq!(second.next().unwrap().payload, i);
        assert_eq!(advanced_count.get(), i + 1);
        assert!(elems[..i + 1].iter().all(|e| e.cloned_count() == 2));
        assert!(elems[i + 1..].iter().all(|e| e.cloned_count() == 1));
    }

    for _ in 0..3 {
        assert!(second.next().is_none());
        assert_eq!(advanced_count.get(), 6);
        assert!(elems.iter().all(|e| e.cloned_count() == 2));
    }

    for i in 2..5 {
        assert_eq!(first.next().unwrap().payload, i);
        assert_eq!(advanced_count.get(), 6);
        assert!(elems.iter().all(|e| e.cloned_count() == 2));
    }

    for _ in 0..3 {
        assert!(first.next().is_none());
        assert_eq!(advanced_count.get(), 6);
        assert!(elems.iter().all(|e| e.cloned_count() == 2));
    }
}

#[test]
fn tee_empty() {
    let vec: Vec<i32> = vec![];
    let (tracked_iter, advanced_count) = TrackedIterator::new(vec);
    let (mut first, mut second) = itertools::tee(tracked_iter);

    for _ in 0..5 {
        assert!(first.next().is_none());
        assert_eq!(advanced_count.get(), 1);

        assert!(second.next().is_none());
        assert_eq!(advanced_count.get(), 1);
    }
}

#[test]
fn group_by_simple() {
    #[derive(Debug)]
    struct Int(usize);

    #[derive(PartialEq, Eq, Debug)]
    struct IsOdd(bool);

    let vec = vec![2, 4, 6, 1, 3, 5, 8, 10, 1, 2]
        .into_iter()
        .map(Int)
        .collect::<Vec<_>>();

    let mut num_called = 0;
    let mut iter = itertools::group_by(vec, |x| {
        num_called += 1;
        IsOdd(x.0 % 2 == 1)
    });

    let expected_groups = vec![
        (false, vec![2, 4, 6]),
        (true, vec![1, 3, 5]),
        (false, vec![8, 10]),
        (true, vec![1]),
        (false, vec![2]),
    ];
    for (expected_odd, expected_group) in expected_groups.into_iter() {
        let (is_odd, group) = iter.next().unwrap();
        assert_eq!(is_odd, IsOdd(expected_odd));
        assert_eq!(
            expected_group,
            group.into_iter().map(|x| x.0).collect::<Vec<_>>(),
        );
    }

    assert!(iter.next().is_none());
}

#[test]
fn group_by_empty() {
    assert!(
        itertools::group_by(Vec::<i32>::new(), |x| *x)
            .next()
            .is_none()
    );
}
