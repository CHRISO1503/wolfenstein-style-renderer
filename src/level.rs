// Width of each cell in matrix is 1 distance unit i.e. (0,0) 1 (1,0) 1 (2,0)... for bottom row
pub const MAP: [[u8; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 3, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 2, 0, 4, 4, 0, 1],
    [1, 0, 0, 0, 4, 0, 0, 1],
    [1, 0, 3, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];
