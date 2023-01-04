// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

fn transpose(matrix: &[&[i32]; 3]) -> [[i32; 3]; 3] {
    let len_r = matrix[0].len();
    let len_c = matrix.len();
    let mut t_matrix = [[0;3];3];
    for i in 0..len_c {
        for j in 0..len_r {
            t_matrix[i][j]=matrix[j][i];
        }
    }
    t_matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        for n in row {
            print!(" {n}")
        }
        println!()
    }
}

fn main() {

    let r1: [i32; 3] = [101, 102, 103];
    let r2: [i32; 3] = [201, 202, 203];
    let r3: [i32; 3] = [301, 302, 303];
    let matrix: [&[i32];3] = [&r1, &r2, &r3];
    let transposed = transpose(&matrix);
    println!("transposed:");
    pretty_print(&transposed);
}