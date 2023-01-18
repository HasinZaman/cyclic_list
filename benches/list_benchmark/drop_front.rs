use arraydeque::{ArrayDeque, Wrapping};
use arrayvec::ArrayVec;
use circular_queue::CircularQueue;
use criterion::{black_box, Criterion};
use cyclic_data_types::list::List;
use ring_queue::Ring;
use smallvec::SmallVec;
use std::collections::linked_list::LinkedList;

macro_rules! list_drop_front_benchmark {
    ($SIZE: literal, $c: expr, $small_vec: literal) => {
        //cyclic list
        $c.bench_function(
            &format!(
                "Cyclic List (no write_over) (no optimization) - drop_front {}",
                $SIZE
            ),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(move || {
                    while list.len() > 0 {
                        let _ = black_box(list.remove_front());
                    }
                })
            },
        );
        $c.bench_function(
            &format!(
                "Cyclic List (write_over) (no optimization) - drop_front {}",
                $SIZE
            ),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();

                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(move || {
                    while list.len() > 0 {
                        let _ = black_box(list.remove_front());
                    }
                })
            },
        );

        $c.bench_function(
            &format!(
                "Cyclic List (no write_over) (optimization) - drop_front {}",
                $SIZE
            ),
            |b| {
                let mut list: List<$SIZE, usize, false> = List::default();
                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(move || {
                    while list.len() > 0 {
                        let _ = list.remove_front();
                    }
                })
            },
        );
        $c.bench_function(
            &format!(
                "Cyclic List (write_over) (optimization) - drop_front {}",
                $SIZE
            ),
            |b| {
                let mut list: List<$SIZE, usize, true> = List::default();

                for i in 0..$SIZE {
                    let _ = list.push_back(black_box(i));
                }

                b.iter(move || {
                    while list.len() > 0 {
                        let _ = list.remove_front();
                    }
                })
            },
        );

        //vec
        $c.bench_function(&format!("vec (remove(0)) - drop_front {}", $SIZE), |b| {
            let mut list = Vec::new();

            for i in 0..$SIZE {
                list.push(black_box(i));
            }
            b.iter(|| {
                while list.len() > 0 {
                    black_box(list.remove(0));
                }
            })
        });

        //linked list
        $c.bench_function(&format!("linked_list - drop_front {}", $SIZE), |b| {
            let mut list = LinkedList::new();

            for i in 0..$SIZE {
                black_box(list.push_back(black_box(i)));
            }
            b.iter(move || {
                while list.len() > 0 {
                    let _ = black_box(list.pop_front());
                }
            })
        });

        //small vec
        $c.bench_function(&format!("small vec - drop_front {}", $SIZE), |b| {
            let mut list: SmallVec<[_; $small_vec]> = SmallVec::with_capacity($SIZE);

            for i in 0..$SIZE {
                list.push(black_box(i));
            }
            b.iter(move || {
                while list.len() > 0 {
                    black_box(list.remove(0));
                }
            })
        });

        //circular queue
        $c.bench_function(&format!("circular queue - drop_front {}", $SIZE), |b| {
            let mut list = CircularQueue::with_capacity($SIZE);

            for i in 0..$SIZE {
                black_box(list.push(black_box(i)));
            }

            let mut iter = list.iter();
            b.iter(move || {
                while let Some(_) = black_box(iter.next()) {
                    continue;
                }
            })
        });

        //arrayvec
        $c.bench_function(&format!("arrayvec - drop_front {}", $SIZE), |b| {
            let mut list: ArrayVec<usize, $SIZE> = ArrayVec::new();

            for i in 0..$SIZE {
                list.push(black_box(i));
            }
            b.iter(move || {
                while list.len() > 0 {
                    black_box(list.remove(0));
                }
            })
        });

        //ring queue
        $c.bench_function(&format!("ring queue - drop_front {}", $SIZE), |b| {
            let mut list = Ring::with_capacity($SIZE);

            for i in 0..$SIZE {
                list.push(black_box(i));
            }
            b.iter(|| {
                while list.len() > 0 {
                    black_box(list.pop_left());
                }
            })
        });
    };
    ($SIZE: literal, $c: expr, $small_vec: literal, $array_deque: literal) => {
        list_drop_front_benchmark!($SIZE, $c, $small_vec);
        //array deque
        $c.bench_function(&format!("array deque - drop_front {}", $SIZE), |b| {
            let mut list: ArrayDeque<[usize; $array_deque], Wrapping> = ArrayDeque::new();

            for i in 0..$SIZE {
                list.push_back(black_box(i));
            }

            b.iter(|| {
                while list.len() > 0 {
                    black_box(list.pop_front());
                }
            })
        });
    };
}
pub fn list_drop_front_benchmark(c: &mut Criterion) {
    list_drop_front_benchmark!(8usize, c, 8, 10);
    list_drop_front_benchmark!(16usize, c, 16, 100);
    list_drop_front_benchmark!(32usize, c, 32, 100);
    list_drop_front_benchmark!(64usize, c, 64, 100);
    list_drop_front_benchmark!(128usize, c, 128, 128);
    list_drop_front_benchmark!(256usize, c, 256, 1024);
    list_drop_front_benchmark!(512usize, c, 512, 1024);
    list_drop_front_benchmark!(1024usize, c, 1024, 1024);
    list_drop_front_benchmark!(2048usize, c, 2048);
    list_drop_front_benchmark!(4096usize, c, 4096);
    list_drop_front_benchmark!(8192usize, c, 8192);
    list_drop_front_benchmark!(16384usize, c, 16384);
}
