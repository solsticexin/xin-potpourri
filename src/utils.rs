use std::fmt::Debug;
///该函数将方阵顺时针旋转90°
pub fn rotate_square_matrix_cw_90(square: &mut Vec<Vec<i32>>) {
    let n = square.len();

    // 首先转置矩阵（行列互换）
    for i in 0..n {
        for j in i..n {
            if i != j {
                // square[i][j] = square[i][j] ^ square[j][i];
                // square[j][i] = square[i][j] ^ square[j][i];
                // square[i][j] = square[i][j] ^ square[j][i];
                let a = square[i][j];
                square[i][j] = square[j][i];
                square[j][i] = a;
            }
        }
    }
    // 然后水平翻转每一行
    for i in 0..n {
        for j in 0..n / 2 {
            let temp = square[i][j];
            square[i][j] = square[i][n - 1 - j];
            square[i][n - 1 - j] = temp;
        }
    }
}


pub trait Visit
where Self:Debug,
{
    fn visit(&self){
        println!("打印self{:?}", self);
    }
    fn visit_mut(&mut self);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_square_matrix_cw_90() {
        // 测试3x3矩阵
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

        rotate_square_matrix_cw_90(&mut matrix);

        assert_eq!(matrix, expected);

        // 测试2x2矩阵
        let mut matrix2 = vec![vec![1, 2], vec![3, 4]];

        let expected2 = vec![vec![3, 1], vec![4, 2]];

        rotate_square_matrix_cw_90(&mut matrix2);

        assert_eq!(matrix2, expected2);

        // 测试1x1矩阵
        let mut matrix3 = vec![vec![5]];
        let expected3 = vec![vec![5]];

        rotate_square_matrix_cw_90(&mut matrix3);

        assert_eq!(matrix3, expected3);

        // 测试4x4矩阵
        let mut matrix4 = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];

        let expected4 = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];

        rotate_square_matrix_cw_90(&mut matrix4);

        assert_eq!(matrix4, expected4);
    }
}
