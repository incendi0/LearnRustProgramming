mod breadth_first_search;
mod depth_first_search;
mod toposort;

#[derive(PartialEq, Debug)]
pub enum Color {
    White,
    Gray,
    Black,
}

#[derive(Debug)]
pub struct BfsVertex {
    id: usize,
    color: Color,
    predecessor: usize,
    distance: usize,
}

impl BfsVertex {
    fn new(id: usize) -> BfsVertex {
        BfsVertex {
            id,
            color: Color::White,
            predecessor: usize::MAX,
            distance: usize::MAX,
        }
    }
}

#[derive(Debug)]
pub struct DfsVertex {
    id: usize,
    color: Color,
    predecessor: usize,
    first_visit_time: usize,
    second_visit_time: usize,
}
impl DfsVertex {
    fn new(id: usize) -> DfsVertex {
        DfsVertex {
            id,
            color: Color::White,
            predecessor: usize::MAX,
            first_visit_time: usize::MAX,
            second_visit_time: usize::MAX,
        }
    }
}
