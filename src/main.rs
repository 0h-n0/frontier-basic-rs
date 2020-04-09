use frontier_basic_rs::{Graph, Edge, State, Frontier};

fn main() {
    // let number_of_vertices = 4;
    // let edge_list = vec![Edge::new(1, 2),
    //                      Edge::new(1, 3),
    //                      Edge::new(2, 4),
    //                      Edge::new(3, 4)];
    let number_of_vertices = 9;
    let edge_list = vec![Edge::new(1, 2),
                         Edge::new(2, 3),
                         Edge::new(1, 4),
                         Edge::new(2, 5),
                         Edge::new(3, 6),
                         Edge::new(4, 5),
                         Edge::new(5, 6),
                         Edge::new(4, 7),
                         Edge::new(5, 8),
                         Edge::new(6, 9),
                         Edge::new(7, 8),
                         Edge::new(8, 9),
    ];
    // let number_of_vertices = 6;
    // let edge_list = vec![Edge::new(1, 2),
    //                      Edge::new(2, 3),
    //                      Edge::new(1, 4),
    //                      Edge::new(2, 5),
    //                      Edge::new(3, 6),
    //                      Edge::new(4, 5),
    //                      Edge::new(5, 6),
    //                      Edge::new(1, 5),
    //                      Edge::new(2, 6),
    // ];


    let g = Graph::new(number_of_vertices, edge_list);
    let state = State::new(g, vec![1, 2], vec![9]);
    let frontier = Frontier::new();
    let mut zdd = frontier.construct(&state);
    println!("zdd number of nodes: {:?}", &zdd.get_number_of_nodes());
    println!("zdd number of solutions: {:?}", zdd.get_number_of_solutions());
    for i in 0..10 {
        println!("zdd number of solutions: {:?}", zdd.get_sample(i));
    }
    println!("{:?}", zdd);
}
