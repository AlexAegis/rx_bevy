# [operator_merge_all](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_merge_all)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_merge_all.svg)](https://crates.io/crates/rx_core_operator_merge_all)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_merge_all)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_merge_all)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

## Example

```sh
cargo run -p rx_core_operator_merge_all --example merge_all_example
```

```rs
#[derive(Clone, Debug)]
enum Either {
    Left,
    Right,
}

let mut upstream_subject = PublishSubject::<Either>::default();
let mut inner_left_subject = PublishSubject::<i32>::default();
let mut inner_right_subject = PublishSubject::<i32>::default();

let l = inner_left_subject.clone();
let r = inner_right_subject.clone();
let mut _subscription = upstream_subject
    .clone()
    .finalize(|| println!("finalize: upstream"))
    .tap_next(|n| println!("emit (source): {n:?}"))
    .map(move |next| match next {
        Either::Left => l.clone(),
        Either::Right => r.clone(),
    })
    .merge_all(usize::MAX, Never::map_into())
    .finalize(|| println!("finalize: downstream"))
    .subscribe(PrintObserver::new("merge_map"));

upstream_subject.next(Either::Left);
inner_left_subject.next(1);
inner_right_subject.next(2);
inner_left_subject.next(3);
inner_right_subject.next(4);
upstream_subject.next(Either::Right);
inner_left_subject.next(5);
inner_right_subject.next(6);
inner_left_subject.next(7);
inner_right_subject.next(8);
inner_left_subject.complete();
inner_left_subject.next(9);
inner_right_subject.next(10);
inner_right_subject.complete();
upstream_subject.complete();
```

Output:

```txt
emit (source): Left
merge_map - next: 1
merge_map - next: 3
emit (source): Right
merge_map - next: 5
merge_map - next: 6
merge_map - next: 7
merge_map - next: 8
merge_map - next: 10
merge_map - completed
finalize: downstream
finalize: upstream
merge_map - unsubscribed
```
