mod nodes;
mod target;
mod target_test;


#[allow(unused)]
struct Graph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}

#[allow(unused)]
struct Node<T> {
    data: T,
    edges: Vec<usize>,
}

#[allow(unused)]
struct Edge<T> {
    data: T,
    nodes: [usize; 2],
}
