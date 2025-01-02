#[derive(Debug)]
pub(crate) struct Grid<T> {
    pub(crate) data: Vec<Vec<T>>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl<T: Default + Clone + From<char>> Grid<T> {
    pub(crate) fn from_string(input: &str) -> Self {
        let data: Vec<Vec<T>> = input
            .lines()
            .map(|line| line.chars().map(T::from).collect())
            .collect();
        let height = data.len();
        let width = data.get(0).map_or(0, |row| row.len());
        Grid {
            data,
            width,
            height,
        }
    }
}

