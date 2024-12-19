use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use glam::IVec2;

pub fn read_input(path_str: &str) -> io::Result<Vec<String>> {
    let path = Path::new(path_str);

    let mut input_file = File::open(path)?;
    let mut content = String::new();
    input_file.read_to_string(&mut content)?;

    Ok(content
        .lines()
        .map(|str_slice| str_slice.to_owned())
        .collect::<Vec<String>>())
}

#[derive(Clone, Debug)]
pub struct Mat<T> {
    pub data: Vec<Vec<T>>,
    pub size: IVec2,
}

impl<T> Mat<T>
where
    T: Clone + Eq,
{
    pub fn new() -> Self {
        Mat {
            data: vec![],
            size: IVec2::new(0, 0),
        }
    }

    pub fn add_row(&mut self, new_row: Vec<T>) {
        self.size.x += 1;
        self.size.y = new_row.len() as i32;
        self.data.push(new_row);
    }

    pub fn is_coord_valid(&self, coords: &IVec2) -> bool {
        coords.min_element() >= 0 && coords.x < self.size.x && coords.y < self.size.y
    }

    pub fn at(&self, coords: &IVec2) -> Option<&T> {
        if self.is_coord_valid(&coords) {
            Some(&self.data[coords.x as usize][coords.y as usize])
        } else {
            None
        }
    }

    pub fn set_at(&mut self, coords: &IVec2, new_value: &T) {
        if self.is_coord_valid(&coords) {
            self.data[coords.x as usize][coords.y as usize] = new_value.clone();
        }
    }

    pub fn find(&self, looking_for: &T) -> Option<IVec2> {
        for (i, row) in self.data.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if *item == *looking_for {
                    return Some(IVec2::new(i as i32, j as i32));
                }
            }
        }

        None
    }
}
