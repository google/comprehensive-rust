fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut mat: [[i32; 3]; 3] = matrix;
    for i in 0..3 {
        for j in i + 1..3 {
            let tmp = mat[i][j];
            mat[i][j] = mat[j][i];
            mat[j][i] = tmp;
        }
    }
    mat
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for arr in matrix {
        for x in arr {
            print!("{x} ")
        }
        println!();
    }
}

fn pretty_print_one_line(line: &[i32]) {
    println!("{line:?}")
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    pretty_print_one_line(&matrix[0]);

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}