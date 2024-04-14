mod median;
mod mode;
mod sorted_slice;

use median::Median;
use mode::Mode;
use sorted_slice::SortedSlice;
use std::any;

fn test_median() -> bool {
    println!("Testing median.");

    macro_rules! generate_test_cases_for {
        (type: $type:ty) => {{
            static MIN_VAL: $type = <$type>::MIN;
            static MAX_VAL: $type = <$type>::MAX;

            let test_cases: Vec<(Vec<$type>, Option<$type>)> = vec![
                (vec![], None),
                (vec![3, 1, 6, 3, 8, 7, 9], Some(6)),
                (vec![1, 4, 6, 3, 2, 5, 8, 7], Some(4)),
                (vec![MAX_VAL, MAX_VAL], Some(MAX_VAL)),
                (vec![MIN_VAL, MIN_VAL], Some(MIN_VAL)),
                (vec![MAX_VAL, 4], Some(4 + (MAX_VAL - 4) / 2)),
                (vec![MIN_VAL, 4], Some((MIN_VAL + 4) / 2)),
            ];

            test_cases
        }};
    }

    macro_rules! generate_test_cases_with_negative_values_for {
        (type: $type:ty) => {{
            static MIN_VAL: $type = <$type>::MIN;
            static MAX_VAL: $type = <$type>::MAX;

            let test_cases: Vec<(Vec<$type>, Option<$type>)> = vec![
                (vec![MAX_VAL, -4], Some((MAX_VAL - 4) / 2)),
                (vec![MIN_VAL, -4], Some(MIN_VAL + (-4 - MIN_VAL) / 2)),
            ];

            test_cases
        }};
    }

    macro_rules! execute_test_cases {
        ($test_cases:ident for type $type:ty) => {{
            let mut all_tests_passed = true;

            for (val_to_test, expected_result) in $test_cases {
                let actual_result = val_to_test.median();

                if actual_result.eq(&expected_result) {
                    println!("Result for {val_to_test:?} (as {}) is {actual_result:?} which is CORRECT.", any::type_name::<$type>());
                } else {
                    println!("Result for {val_to_test:?} (as {}) is {actual_result:?} which is INCORRECT. Expected {expected_result:?}.", any::type_name::<$type>());

                    all_tests_passed = false;
                    break;
                }
            }

            all_tests_passed
        }};
    }

    macro_rules! generate_test_func_for {
        (type: $type:ty) => {
            || {
                let test_cases = generate_test_cases_for!(type: $type);
                execute_test_cases!(test_cases for type $type)
            }
        };
    }

    macro_rules! generate_test_func_with_negative_values_for {
        (type: $type:ty) => {
            || {
                let test_cases = generate_test_cases_with_negative_values_for!(type: $type);
                execute_test_cases!(test_cases for type $type)
            }
        };
    }

    let test_funcs = [
        generate_test_func_for!(type: i8),
        generate_test_func_with_negative_values_for!(type: i8),
        generate_test_func_for!(type: u8),
        generate_test_func_for!(type: i16),
        generate_test_func_with_negative_values_for!(type: i16),
        generate_test_func_for!(type: u16),
        generate_test_func_for!(type: i32),
        generate_test_func_with_negative_values_for!(type: i32),
        generate_test_func_for!(type: u32),
        generate_test_func_for!(type: i64),
        generate_test_func_with_negative_values_for!(type: i64),
        generate_test_func_for!(type: u64),
        generate_test_func_for!(type: i128),
        generate_test_func_with_negative_values_for!(type: i128),
        generate_test_func_for!(type: u128),
    ];

    for test_func in test_funcs {
        if !test_func() {
            println!("A median test failed.");
            return false;
        }
    }

    println!("All median tests passed.");
    true
}

#[allow(clippy::too_many_lines)]
fn test_mode_for_sorted_slice() -> bool {
    println!("Testing mode for SortedSlice.");

    macro_rules! generate_test_func_for {
        (type: $type:ty) => {
            || {
                enum SortedSliceConstructionKind {
                    FromSorted,
                    FromUnsorted
                }

                let test_cases: Vec<(Vec<$type>, SortedSliceConstructionKind, Option<Vec<$type>>)> = vec![
                    (
                        vec![],
                        SortedSliceConstructionKind::FromSorted,
                        None
                    ),
                    (
                        vec![1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1])
                    ),
                    (
                        vec![1, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1])
                    ),
                    (
                        vec![1, 2],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1, 2])
                    ),
                    (
                        vec![2, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![2, 1])
                    ),
                    (
                        vec![1, 1, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1])
                    ),
                    (
                        vec![1, 2, 2],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![2])
                    ),
                    (
                        vec![2, 2, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![2])
                    ),
                    (
                        vec![2, 1, 2],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![2])
                    ),
                    (
                        vec![1, 1, 1, 2],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1])
                    ),
                    (
                        vec![2, 1, 1, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1])
                    ),
                    (
                        vec![1, 2, 1, 1],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![1])
                    ),
                    (
                        vec![1, 2, 2, 2],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![2])
                    ),
                    (
                        vec![2, 2, 2, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![2])
                    ),
                    (
                        vec![2, 2, 1, 2],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![2])
                    ),
                    (
                        vec![1, 2, 3, 3],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![3])
                    ),
                    (
                        vec![3, 3, 2, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![3])
                    ),
                    (
                        vec![3, 2, 1, 3],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![3])
                    ),
                    (
                        vec![1, 1, 1, 2, 2],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1])
                    ),
                    (
                        vec![2, 2, 1, 1, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1])
                    ),
                    (
                        vec![1, 2, 1, 2, 1],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![1]),
                    ),
                    (
                        vec![1, 1, 1, 2, 2, 2],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1, 2]),
                    ),
                    (
                        vec![2, 2, 2, 1, 1, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![2, 1]),
                    ),
                    (
                        vec![1, 2, 1, 2, 1, 2],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![1, 2]),
                    ),
                    (
                        vec![1, 1, 1, 2, 2, 2, 3],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1, 2]),
                    ),
                    (
                        vec![3, 2, 2, 2, 1, 1, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![2, 1]),
                    ),
                    (
                        vec![2, 1, 3, 2, 1, 2, 1],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![1, 2]),
                    ),
                    (
                        vec![1, 1, 1, 2, 2, 2, 3, 3],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1, 2]),
                    ),
                    (
                        vec![3, 3, 2, 2, 2, 1, 1, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![2, 1]),
                    ),
                    (
                        vec![1, 2, 1, 3, 2, 2, 1, 3],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![1, 2]),
                    ),
                    (
                        vec![1, 1, 1, 2, 2, 2, 3, 3, 3],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1, 2, 3]),
                    ),
                    (
                        vec![3, 3, 3, 2, 2, 2, 1, 1, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![3, 2, 1]),
                    ),
                    (
                        vec![3, 2, 1, 2, 3, 1, 3, 1, 2],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![1, 2, 3]),
                    ),
                    (
                        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
                    ),
                    (
                        vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![9, 8, 7, 6, 5, 4, 3, 2, 1]),
                    ),
                    (
                        vec![6, 3, 2, 5, 9, 4, 1, 8, 7],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
                    ),
                    (
                        vec![1, 2, 3, 4, 5, 6, 7, 8, 8],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![8]),
                    ),
                    (
                        vec![8, 8, 7, 6, 5, 4, 3, 2, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![8]),
                    ),
                    (
                        vec![7, 5, 8, 2, 8, 4, 1, 3, 6],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![8]),
                    ),
                    (
                        vec![1, 2, 3, 4, 5, 6, 6, 7, 7],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![6, 7]),
                    ),
                    (
                        vec![7, 7, 6, 6, 5, 4, 3, 2, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![7, 6]),
                    ),
                    (
                        vec![5, 6, 7, 1, 3, 7, 4, 6, 2],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![6, 7]),
                    ),
                    (
                        vec![1, 2, 3, 4, 5, 6, 6, 7, 8],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![6]),
                    ),
                    (
                        vec![8, 7, 6, 6, 5, 4, 3, 2, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![6]),
                    ),
                    (
                        vec![4, 3, 6, 7, 8, 2, 5, 6, 1],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![6]),
                    ),
                    (
                        vec![1, 2, 2, 3, 4, 5, 6, 7, 8],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![2]),
                    ),
                    (
                        vec![8, 7, 6, 5, 4, 3, 2, 2, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![2]),
                    ),
                    (
                        vec![2, 1, 8, 3, 4, 7, 6, 5, 2],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![2]),
                    ),
                    (
                        vec![1, 1, 2, 3, 4, 5, 6, 7, 8],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1]),
                    ),
                    (
                        vec![8, 7, 6, 5, 4, 3, 2, 1, 1],
                        SortedSliceConstructionKind::FromSorted,
                        Some(vec![1]),
                    ),
                    (
                        vec![8, 1, 4, 6, 3, 1, 5, 7, 2],
                        SortedSliceConstructionKind::FromUnsorted,
                        Some(vec![1]),
                    ),
                ];

                for (mut val_to_test, sorted_slice_construction_kind, expected_result) in test_cases {
                    let original_val = val_to_test.clone();
                    let actual_result = match sorted_slice_construction_kind {
                        SortedSliceConstructionKind::FromSorted => SortedSlice::from_sorted(&val_to_test).mode(),
                        SortedSliceConstructionKind::FromUnsorted => SortedSlice::from_unsorted(&mut val_to_test).mode()
                    };

                    if actual_result.eq(&expected_result) {
                        println!("Result for {original_val:?} (as {}, sorted as {val_to_test:?}) is {actual_result:?} which is CORRECT.", any::type_name::<$type>());
                    } else {
                        println!("Result for {original_val:?} (as {}, sorted as {val_to_test:?}) is {actual_result:?} which is INCORRECT. Expected {expected_result:?}.", any::type_name::<$type>());
                        return false;
                    }
                }

                true
            }
        };
    }

    let test_funcs = [
        generate_test_func_for!(type: i8),
        generate_test_func_for!(type: u8),
        generate_test_func_for!(type: i16),
        generate_test_func_for!(type: u16),
        generate_test_func_for!(type: i32),
        generate_test_func_for!(type: u32),
        generate_test_func_for!(type: i64),
        generate_test_func_for!(type: u64),
        generate_test_func_for!(type: i128),
        generate_test_func_for!(type: u128),
    ];

    for test_func in test_funcs {
        if !test_func() {
            println!("A mode test for SortedSlice failed.");
            return false;
        }
    }

    println!("All mode tests for SortedSlice passed.");
    true
}

fn test_mode_for_unsorted_slice() -> bool {
    println!("Testing mode for unsorted slice.");

    macro_rules! generate_test_func_for {
        (type: $type:ty) => {
            || {
                let test_cases: Vec<(Vec<$type>, Option<Vec<$type>>)> = vec![
                    (vec![], None),
                    (vec![1], Some(vec![1])),
                    (vec![1, 2], Some(vec![1, 2])),
                    (vec![2, 1], Some(vec![2, 1])),
                    (vec![1, 2, 3, 3], Some(vec![3])),
                    (vec![3, 3, 1, 2], Some(vec![3])),
                    (vec![1, 2, 1, 2], Some(vec![1, 2])),
                    (vec![1, 1, 2, 2], Some(vec![1, 2])),
                    (vec![1, 1, 2, 1], Some(vec![1])),
                    (vec![1, 1, 2, 2, 3, 3, 1], Some(vec![1])),
                    (vec![1, 2, 2, 2, 3, 3, 1, 1, 1], Some(vec![1])),
                    (vec![1, 2, 2, 2, 3, 3, 3, 1, 1], Some(vec![2, 3, 1])),
                    (vec![1, 2, 2, 2, 3, 3, 3, 1, 1, 1, 4, 1], Some(vec![1])),
                ];

                for (val_to_test, expected_result) in test_cases {
                    let actual_result = val_to_test.mode();

                    if actual_result.eq(&expected_result) {
                        println!("Result for {val_to_test:?} (as {}) is {actual_result:?} which is CORRECT.", any::type_name::<$type>());
                    } else {
                        println!("Result for {val_to_test:?} (as {}) is {actual_result:?} which is INCORRECT. Expected {expected_result:?}.", any::type_name::<$type>());
                        return false;
                    }
                }

                true
            }
        };
    }

    let test_funcs = [
        generate_test_func_for!(type: i8),
        generate_test_func_for!(type: u8),
        generate_test_func_for!(type: i16),
        generate_test_func_for!(type: u16),
        generate_test_func_for!(type: i32),
        generate_test_func_for!(type: u32),
        generate_test_func_for!(type: i64),
        generate_test_func_for!(type: u64),
        generate_test_func_for!(type: i128),
        generate_test_func_for!(type: u128),
    ];

    for test_func in test_funcs {
        if !test_func() {
            println!("A mode test for unsorted slice failed.");
            return false;
        }
    }

    println!("All mode tests for unsorted slice passed.");
    true
}

fn main() {
    let test_funcs = [
        test_median,
        test_mode_for_sorted_slice,
        test_mode_for_unsorted_slice,
    ];

    for test_func in test_funcs {
        if !test_func() {
            println!("A test failed.");
            return;
        }
    }

    println!("All tests passed.");
}
