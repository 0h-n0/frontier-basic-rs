use frontier_basic_rs::{Graph, Edge, State, Frontier};

fn main() {
    let number_of_vertices = 4;
    let edge_list = vec![Edge::new(0, 1),
                         Edge::new(0, 2),
                         Edge::new(1, 3),
                         Edge::new(2, 3)];

    let g = Graph::new(number_of_vertices, edge_list);
    let state = State::new(g, 0, 3);
    let frontier = Frontier::new();
    let mut zdd = frontier.construct(&state);
    println!("zdd number of nodes: {:?}", &zdd.get_number_of_nodes());
    println!("zdd number of solutions: {:?}", zdd.get_number_of_solutions());
    //println!("{:?}", zdd);
}
