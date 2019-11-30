pub fn flatten_2d<T>(v: &Vec<Vec<T>>) -> Vec<(usize, usize, &T)> {
    v.iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(column_index, tile_id)| (row_index, column_index, tile_id))
                .collect::<Vec<(usize, usize, &T)>>()
        })
        .collect()
}
