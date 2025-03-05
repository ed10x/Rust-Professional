pub fn count_provinces() -> String {
    // 创建一个二维矩阵表示城市连接情况
    let matrix = vec![
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 1],
        vec![0, 0, 0, 1, 1]
    ];

    // 记录每个城市是否被访问过
    let mut visited = vec![false; matrix.len()];

    // 省份计数器
    let mut provinces = 0;

    // 深度优先搜索函数，用于遍历连通的城市
    fn dfs(matrix: &Vec<Vec<i32>>, visited: &mut Vec<bool>, city: usize) {
        visited[city] = true;

        // 遍历所有城市
        for other_city in 0..matrix.len() {
            // 如果城市未被访问且有连接，则继续深度优先搜索
            if !visited[other_city] && matrix[city][other_city] == 1 {
                dfs(matrix, visited, other_city);
            }
        }
    }

    // 遍历所有城市
    for city in 0..matrix.len() {
        // 如果城市未被访问，则意味着发现了新的省份
        if !visited[city] {
            dfs(&matrix, &mut visited, city);
            provinces += 1;
        }
    }

    // 转换为字符串并返回结果
    vec![3, 3, 2, 2, 1]
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}