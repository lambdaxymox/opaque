use opaque_index_map_testing as oimt;

#[test]
fn test_last_entry_per_key1() {
    let entries: Vec<(i32, i32)> = vec![];
    let expected = vec![];
    let result = oimt::map::last_entry_per_key_ordered(&entries);

    assert_eq!(result, expected);
}

#[test]
fn test_last_entry_per_key2() {
    let entries = vec![(1, 2)];
    let expected = vec![(1, 2)];
    let result = oimt::map::last_entry_per_key_ordered(&entries);

    assert_eq!(result, expected);
}

#[test]
fn test_last_entry_per_key3() {
    let entries = vec![(1, 2), (1, 3), (1, 4)];
    let expected = vec![(1, 4)];
    let result = oimt::map::last_entry_per_key_ordered(&entries);

    assert_eq!(result, expected);
}

#[test]
fn test_last_entry_per_key4() {
    let entries = vec![(1, 2), (1, 3), (1, 4), (2, 5)];
    let expected = vec![(1, 4), (2, 5)];
    let result = oimt::map::last_entry_per_key_ordered(&entries);

    assert_eq!(result, expected);
}

#[test]
fn test_last_entry_per_key5() {
    let entries = vec![
        (1, 2),
        (1, 3),
        (1, 4),
        (2, 5),
        (3, 6),
        (3, 7),
        (4, 8),
        (4, 9),
        (4, 10),
        (4, 11),
        (4, 12),
        (5, 13),
        (5, 14),
        (5, 15),
        (6, 16),
        (7, 17),
        (8, 18),
    ];
    let expected = vec![(1, 4), (2, 5), (3, 7), (4, 12), (5, 15), (6, 16), (7, 17), (8, 18)];
    let result = oimt::map::last_entry_per_key_ordered(&entries);

    assert_eq!(result, expected);
}

#[test]
fn test_last_entry_per_key6() {
    let entries = vec![
        (1, 2),
        (1, 3),
        (1, 4),
        (2, 5),
        (3, 6),
        (3, 7),
        (4, 8),
        (4, 9),
        (4, 10),
        (4, 11),
        (4, 12),
        (5, 13),
        (5, 14),
        (5, 15),
        (6, 16),
        (7, 17),
        (8, 18),
        (9, 19),
        (9, 20),
        (9, 21),
        (9, 22),
        (9, 23),
        (9, 24),
        (9, 25),
        (9, 26),
        (10, 27),
        (10, 28),
        (10, 29),
        (10, 30),
        (11, 31),
        (12, 32),
        (13, 33),
        (14, 34),
        (15, 35),
        (15, 36),
        (15, 37),
        (15, 38),
    ];
    let expected = vec![
        (1, 4),
        (2, 5),
        (3, 7),
        (4, 12),
        (5, 15),
        (6, 16),
        (7, 17),
        (8, 18),
        (9, 26),
        (10, 30),
        (11, 31),
        (12, 32),
        (13, 33),
        (14, 34),
        (15, 38),
    ];
    let result = oimt::map::last_entry_per_key_ordered(&entries);

    assert_eq!(result, expected);
}

#[test]
fn test_last_entry_per_key7() {
    let entries = vec![
        (1, 2),
        (1, 3),
        (1, 4),
        (9, 19),
        (9, 20),
        (9, 21),
        (9, 22),
        (9, 23),
        (2, 5),
        (3, 6),
        (3, 7),
        (4, 8),
        (4, 9),
        (4, 10),
        (4, 11),
        (4, 12),
        (5, 13),
        (5, 14),
        (5, 15),
        (6, 16),
        (10, 27),
        (10, 28),
        (10, 29),
        (7, 17),
        (8, 18),
        (9, 24),
        (9, 25),
        (9, 26),
        (10, 30),
        (11, 31),
        (15, 35),
        (15, 36),
        (12, 32),
        (13, 33),
        (14, 34),
        (15, 37),
        (15, 38),
    ];
    let expected = vec![
        (1, 4),
        (9, 26),
        (2, 5),
        (3, 7),
        (4, 12),
        (5, 15),
        (6, 16),
        (10, 30),
        (7, 17),
        (8, 18),
        (11, 31),
        (15, 38),
        (12, 32),
        (13, 33),
        (14, 34),
    ];
    let result = oimt::map::last_entry_per_key_ordered(&entries);

    assert_eq!(result, expected);
}

#[test]
fn test_last_entry_per_key8() {
    let entries = vec![
        (15, 38),
        (14, 34),
        (13, 33),
        (12, 32),
        (11, 31),
        (10, 30),
        (9, 26),
        (8, 18),
        (7, 17),
        (6, 16),
        (5, 15),
        (4, 12),
        (3, 7),
        (2, 5),
        (1, 4),
    ];
    let expected = vec![
        (15, 38),
        (14, 34),
        (13, 33),
        (12, 32),
        (11, 31),
        (10, 30),
        (9, 26),
        (8, 18),
        (7, 17),
        (6, 16),
        (5, 15),
        (4, 12),
        (3, 7),
        (2, 5),
        (1, 4),
    ];
    let result = oimt::map::last_entry_per_key_ordered(&entries);

    assert_eq!(result, expected);
}
