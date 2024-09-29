fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut row1: Vec<i32> = Vec::with_capacity(3);
    let mut row2: Vec<i32> = Vec::with_capacity(3);
    let mut row3: Vec<i32> = Vec::with_capacity(3);
    for array in matrix{
        row1.push(array[0]);
        row2.push(array[1]);
        row3.push(array[2]);
    }
    let row1: [i32; 3] = [row1[0], row1[1], row1[2]];
    let row2: [i32; 3] = [row2[0], row2[1], row2[2]];
    let row3: [i32; 3] = [row3[0], row3[1], row3[2]];
    let transpose = [row1, row2, row3];
    transpose // This exercise is a lesson for anyone who wants to use arrays
    // in Rust like the way they use them in Python3, just don't...
    // Vectors are what you want, arrays are inflexible vectors
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}