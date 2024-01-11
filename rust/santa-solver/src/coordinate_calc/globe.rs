/*
* Globe has m horizontal and n vertical cuts, so it has (m + 1) * 2 * n faces
*/
pub fn get_globe_order_to_traverse(m: usize, n: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    // for col_num in 0..(2 * n) {
    //     for row_num in 0..(m + 1) {
    //         let cur_face = row_num * 2 * n + col_num;
    //         result.push(cur_face);
    //     }
    // }
    // bottom, then top
    let half = (m + 1) * n - 1;
    for i in 0..=half {
        result.push(i);
        result.push((2 * n) * (m + 1) - 1 - i);
    }
    return result;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_get_globe_order_to_traverse() {
        let m = 2;
        let n = 6;
        let result = get_globe_order_to_traverse(m, n);
        let expected = vec![
            0, 12, 24, 1, 13, 25, 2, 14, 26, 3, 15, 27, 4, 16, 28, 5, 17, 29, 6, 18, 30, 7, 19, 31,
            8, 20, 32, 9, 21, 33, 10, 22, 34, 11, 23, 35,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_globe_order_to_traverse_1_16() {
        let m = 1;
        let n = 16;
        let result = get_globe_order_to_traverse(m, n);
        let expected = vec![
            0, 12, 24, 1, 13, 25, 2, 14, 26, 3, 15, 27, 4, 16, 28, 5, 17, 29, 6, 18, 30, 7, 19, 31,
            8, 20, 32, 9, 21, 33, 10, 22, 34, 11, 23, 35,
        ];
        assert_eq!(result, expected);
    }
}
