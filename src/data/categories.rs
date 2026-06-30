pub const DRIVER_CATEGORIES: &[(u8, &str)] = &[
    (0, "Bronze"),
    (1, "Silver"),
    (2, "Gold"),
    (3, "Platinum"),
];

pub const CUP_CATEGORIES: &[(u8, &str)] = &[
    (0, "Overall"),
    (1, "Pro-Am"),
    (2, "Am"),
    (3, "Silver Cup"),
    (4, "National Am"),
];

pub const SESSION_TYPES: &[(&str, &str)] = &[
    ("P", "Practice"),
    ("Q", "Qualifying"),
    ("R", "Race"),
];
