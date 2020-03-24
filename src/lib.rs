// ref: https://github.com/junkawahara/frontier-basic
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
struct Edge {
    src: i64,
    dst: i64,
}

#[derive(Debug, Clone)]
struct Graph {
    number_of_vertices: i64,
    edge_list: Vec<Edge>
}

impl Graph {
    fn new(number_of_vertices: i64, edge_list: Vec<Edge>) -> Self {
        Graph {
            number_of_vertices,
            edge_list
        }
    }
    fn get_number_of_vertices(&self) -> i64 {
        self.number_of_vertices
    }
    fn get_edge_list(&self) -> &Vec<Edge> {
        &self.edge_list
    }
    fn parse_adj_list_text(self) {
    }
    fn to_string(self) {
    }
}

impl Edge {
    fn new(src: i64, dst: i64) -> Self {
        Edge { src, dst }
    }
}

#[derive(Debug, Default, Clone)]
struct ZDDNode {
    deg: Option<Vec<i64>>,
    comp: Option<Vec<i64>>,
    sol: i64,
    zero_child: Option<std::rc::Rc<ZDDNode>>,
    one_child: Option<std::rc::Rc<ZDDNode>>,
    id: i64,
}

trait ZDDNodeTrait {
    const total_id: std::cell::RefCell<i64> = std::cell::RefCell::new(2);
    const zero_t: ZDDNode = ZDDNode {
            deg: None,
            comp: None,
            sol: 0,
            zero_child: None,
            one_child: None,
            id: 0,
    };
    const one_t: ZDDNode = ZDDNode {
            deg: None,
            comp: None,
            sol: 1,
            zero_child: None,
            one_child: None,
            id: 1,
    };
    fn set_next_id() -> i64;
    fn create_root_node(number_of_vertices: i64) -> Self;
    fn get_id(self) -> i64;
    fn make_copy(self, number_of_vertices: i64) -> Self;
    fn set_child(&mut self, node: Self, child_num: i64);
    fn get_child(&self, child_num: i64) -> std::rc::Rc<Self>;
}

impl ZDDNodeTrait for ZDDNode {
    fn set_next_id() -> i64 {
        let id = ZDDNode::total_id.into_inner();
        ZDDNode::total_id.replace(id + 1);
        return id
    }
    fn create_root_node(number_of_vertices: i64) -> Self {
        let mut deg = vec![];
        let mut comp = vec![];

        for i in 1..=number_of_vertices {
            deg.push(0);
            comp.push(i);
        }
        ZDDNode {
            deg: Some(deg),
            comp: Some(comp),
            sol: 0,
            zero_child: None,
            one_child: None,
            id: ZDDNode::set_next_id(),
        }
    }
    fn get_id(self) -> i64 {
        self.id
    }
    fn make_copy(self, number_of_vertices: i64) -> Self {
        let mut deg = vec![];
        let mut comp = vec![];

        let self_deg = self.deg.unwrap();
        let self_comp = self.comp.unwrap();
        for i in 1..=number_of_vertices {
            deg.push(self_deg[i as usize]);
            comp.push(self_comp[i as usize]);
        }
        ZDDNode {
            deg: Some(deg),
            comp: Some(comp),
            sol: 0,
            zero_child: None,
            one_child: None,
            id: ZDDNode::set_next_id(),
        }
    }
    fn set_child(&mut self, node: Self, child_num: i64) {
        if child_num == 0 {
            self.zero_child = Some(std::rc::Rc::new(node));
        } else {
            self.one_child = Some(std::rc::Rc::new(node));
        }
    }
    fn get_child(&self, child_num: i64) -> std::rc::Rc<Self> {
        if child_num == 0 {
            (&self.zero_child).as_ref().unwrap().clone()
        } else {
            (&self.one_child).as_ref().unwrap().clone()
        }
    }
}

pub struct State {
    graph: std::rc::Rc<Graph>,
    s: i64,
    t: i64,
    frontier: Vec<Vec<i64>>,
}

impl State {
    fn new(graph: Graph, start: i64,  end: i64) -> Self {
        let graph = std::rc::Rc::new(graph);
        State {
            s: start,
            t: end,
            graph: graph.clone(),
            frontier: State::compute_frontier(graph.clone()),
        }
    }
    fn compute_frontier(graph: std::rc::Rc<Graph>) -> Vec<Vec<i64>>{
        let edge_list = graph.get_edge_list();
        let mut frontier = vec![vec![]; edge_list.len() + 1];

        for i in 0..edge_list.len() {
            for j in 0..frontier[i].len() {
                let a = frontier[i][j];
                frontier[i + 1].push(a);
            }
            let edge = &edge_list[i];
            let src = edge.src;
            let dst = edge.dst;
            if !frontier[i + 1].contains(&src) {
                frontier[i + 1].push(src)
            }
            if !frontier[i + 1].contains(&dst) {
                frontier[i + 1].push(dst)
            }
            if !State::find_element(graph.clone(), i, src) {
                let mut v = frontier[i].clone().into_iter().filter(|&i| i != src).collect::<Vec<_>>();
                frontier[i].truncate(0);
                frontier[i].append(&mut v);
            }
            if !State::find_element(graph.clone(), i, dst) {
                let mut v = frontier[i].clone().into_iter().filter(|&i| i != dst).collect::<Vec<_>>();
                frontier[i].truncate(0);
                frontier[i].append(&mut v);
            }
        }
        frontier
    }
    fn find_element(graph: std::rc::Rc<Graph>, edge_number: usize, value: i64) -> bool {
        let edge_list = graph.get_edge_list();
        for i in edge_number + 1..edge_list.len() {
            if  value == edge_list[i].src || value == edge_list[i].dst {
                return true
            }
        }
        false
    }
}

struct ZDD {
    node_list_array: Vec<Vec<ZDDNode>>,
}

impl ZDD {
    fn get_number_of_nodes(&self) -> usize {
        let mut num = 0;
        for i in 1..self.node_list_array.len() {
            num += self.node_list_array[i].len()
        }
        num + 2
    }
    fn get_number_of_solutions(&mut self) -> i64 {
        let mut i = self.node_list_array.len();
        while i > 1 {
            for j in 0..self.node_list_array[i].len() {
                let lo_node = self.node_list_array[i][j].get_child(0);
                let hi_node = self.node_list_array[i][j].get_child(1);
                self.node_list_array[i][j].sol = lo_node.sol + hi_node.sol;
            }
        }
        self.node_list_array[1][0].sol
    }
}

pub mod FrontierAlgorithm {
    use super::*;
    pub fn construct(state: &State) {
        let edge_list = state.graph.get_edge_list();
        let mut N = vec![vec![]; edge_list.len() + 2];
        N[1].push(ZDDNode::create_root_node(state.graph.get_number_of_vertices()));
        for i in 1..edge_list.len() {
            for j in 0..N[i].len() {
                let n_hat = &N[i][j];
                for x in 0..i {
                    let n_prime = check_terminal(n_hat, i, x as i64, state);

                }
            }
        }
    }
    fn check_terminal(n_hat: &ZDDNode, i: usize, x: i64, state: &State) {
        let edge_list = &state.graph.get_edge_list()[i - 1];

    }
    fn update_info(n_hat: &mut ZDDNode, i: usize, x: i64, state: &State) {
        let edge = &state.graph.get_edge_list()[i - 1];
        for y in 0..=1 {
            let u = match y {
                0 => edge.src,
                _ => edge.dst,
            };
            if !state.frontier[i - 1].contains(&u) {
                n_hat.deg.as_ref().unwrap()[u] = 0;
                n_hat.deg.as_ref().unwrap()[u] = u;
            }
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zdd_create_root_node_3() {
        let z = ZDDNode::create_root_node(3);
        assert_eq!(z.comp.unwrap(), vec![1, 2, 3]);
        assert_eq!(z.deg.unwrap(), vec![0, 0, 0]);
    }
    #[test]
    fn test_zdd_create_root_node_1() {
        let z = ZDDNode::create_root_node(1);
        assert_eq!(z.comp.unwrap(), vec![1]);
        assert_eq!(z.deg.unwrap(), vec![0]);
    }
    #[test]
    fn test_zdd_create_root_node_2() {
        let z = ZDDNode::create_root_node(2);
        assert_eq!(z.comp.unwrap(), vec![1, 2]);
        assert_eq!(z.deg.unwrap(), vec![0, 0]);
    }
    #[test]
    fn test_zdd_create_root_node_edge_case() {
        let z = ZDDNode::create_root_node(0);
        assert_eq!(z.comp.unwrap(), vec![]);
        assert_eq!(z.deg.unwrap(), vec![]);
    }

    #[test]
    fn graph_get_number_of_vertices() {
        let e1 = Edge::new(0, 1);
        let e2 = Edge::new(0, 2);
        let e3 = Edge::new(1, 3);
        let e4 = Edge::new(2, 3);
        let edge_list = vec![e1, e2, e3, e4];
        let g = Graph::new(4, edge_list.clone());
        assert_eq!(g.get_number_of_vertices(), 4);
        assert_eq!(g.get_edge_list(), &edge_list);
    }
    #[test]
    fn zdd_get_child() {
        let z1: ZDDNode = ZDDNode {
            deg: None,
            comp: None,
            sol: 0,
            zero_child: None,
            one_child: None,
            id: 0,
        };
        let z2: ZDDNode = ZDDNode {
            deg: None,
            comp: None,
            sol: 0,
            zero_child: None,
            one_child: None,
            id: 1,
        };
        let z3: ZDDNode = ZDDNode {
            deg: None,
            comp: None,
            sol: 0,
            zero_child: Some(std::rc::Rc::new(z1)),
            one_child: Some(std::rc::Rc::new(z2)),
            id: 2,
        };
        let zero = (&z3).get_child(0);
        assert_eq!(zero.id, 0);
        assert_eq!(zero.sol, 0);
        assert_eq!(zero.id, 0);
        assert_eq!(zero.sol, 0);
    }
}
