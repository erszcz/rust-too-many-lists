//pub enum List {
//    Empty,
//    Elem(i32, Box<List>)
//}

pub struct List {
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>)
}

struct Node {
    elem: i32,
    next: Link
}
