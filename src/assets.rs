pub const FRAMES: [&[&str]; 3] = [
    &[
        include_str!("../assets/v1/1.txt"), // 0 
        include_str!("../assets/v1/2.txt"), // 1
        include_str!("../assets/v1/3.txt"), // 2
        include_str!("../assets/v1/4.txt"), // 3
        include_str!("../assets/v1/5.txt"), // 4
    ],
    &[
        include_str!("../assets/v2/1.txt"), // 5
        include_str!("../assets/v2/2.txt"), // 6
        include_str!("../assets/v2/3.txt"), // 7
        include_str!("../assets/v2/4.txt"), // 8
        include_str!("../assets/v2/5.txt"), // 9
    ],
    &[
        include_str!("../assets/v3/1.txt"), // 10
        include_str!("../assets/v3/2.txt"), // 11
        include_str!("../assets/v3/3.txt"), // 12
        include_str!("../assets/v3/4.txt"), // 13
        include_str!("../assets/v3/5.txt"), // 14
        include_str!("../assets/v3/6.txt"), // 15
        include_str!("../assets/v3/7.txt"), // 16
        include_str!("../assets/v3/8.txt"), // 17
    ]
];

pub const FRAME_ORDER: [(usize, usize); (5 + 10 + 8) * 3] = [
    (0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
    (0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
    (0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
    (1, 0), (1, 1), (1, 0), (1, 1), (1, 0), (1, 1), (1, 4), (1, 2), (1, 3), (1, 4),
    (1, 0), (1, 1), (1, 0), (1, 1), (1, 0), (1, 1), (1, 4), (1, 2), (1, 3), (1, 4),
    (1, 0), (1, 1), (1, 0), (1, 1), (1, 0), (1, 1), (1, 4), (1, 2), (1, 3), (1, 4),
    (2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6), (2, 7),
    (2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6), (2, 7),
    (2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6), (2, 7),
];
