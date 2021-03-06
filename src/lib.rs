// ref: https://github.com/junkawahara/frontier-basic
// ref: https://users.rust-lang.org/t/is-rc-refcell-a-code-smell/27366
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    src: usize,
    dst: usize,
}

#[derive(Debug, Clone)]
pub struct Graph {
    number_of_vertices: usize,
    edge_list: Vec<Edge>
}

impl Graph {
    pub fn new(number_of_vertices: usize, edge_list: Vec<Edge>) -> Self {
        Graph {
            number_of_vertices,
            edge_list
        }
    }
    fn get_number_of_vertices(&self) -> usize {
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
    pub fn new(src: usize, dst: usize) -> Self {
        Edge { src, dst }
    }
}

#[derive(Debug, Default, Clone)]
struct TotalId {
    id: RefCell<i64>
}

#[derive(Default, Clone)]
struct ZDDNode {
    deg: Option<RefCell<Vec<usize>>>,
    comp: Option<RefCell<Vec<usize>>>,
    sol: i64,
    zero_child: Option<Rc<RefCell<ZDDNode>>>,
    one_child: Option<Rc<RefCell<ZDDNode>>>,
    id: usize,
}

trait ZDDNodeTrait {
    fn create_root_node(number_of_vertices: usize, id: usize) -> Self;
    fn get_id(&self) -> usize;
    fn set_next_id(&mut self, id: usize);
    fn make_copy(&self, number_of_vertices: usize, id: usize) -> Self;
    fn set_child(&mut self, node: Rc<RefCell<Self>>, child_num: usize);
    fn get_child(&self, child_num: i64) -> Rc<RefCell<Self>>;
}

impl std::fmt::Display for ZDDNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let z_id = match self.zero_child.as_ref() {
            Some(v) => v.clone().borrow().id,
            None => 10000,
        };
        let o_id = match self.one_child.as_ref() {
            Some(v) => v.clone().borrow().id,
            None => 10000,
        };
        write!(f, "{}:({}, {})", self.id, z_id, o_id)
    }
}

impl std::fmt::Debug for ZDDNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let z_id = match self.zero_child.as_ref() {
            Some(v) => v.clone().borrow().id,
            None => 10000,
        };
        let o_id = match self.one_child.as_ref() {
            Some(v) => v.clone().borrow().id,
            None => 10000,
        };
        write!(f, "{}:(z: {}, o: {})", self.id, z_id, o_id)
    }
}

impl ZDDNodeTrait for ZDDNode {
    fn create_root_node(number_of_vertices: usize, id: usize) -> Self {
        let deg = vec![0; number_of_vertices + 1];
        let mut comp = vec![0; number_of_vertices + 1];
        for i in 1..=number_of_vertices {
            comp[i] = i;
        }
        ZDDNode {
            deg: Some(RefCell::new(deg)),
            comp: Some(RefCell::new(comp)),
            sol: 0,
            zero_child: None,
            one_child: None,
            id: id,
        }
    }
    fn set_next_id(&mut self, id: usize) {
        self.id = id;
    }
    fn get_id(&self) -> usize {
        self.id
    }
    fn make_copy(&self, number_of_vertices: usize, id: usize) -> Self {
        let mut deg = vec![0; number_of_vertices + 1];
        let mut comp = vec![0; number_of_vertices + 1];
        let self_deg = self.deg.as_ref().unwrap().borrow();
        let self_comp = self.comp.as_ref().unwrap().borrow();
        for i in 1..=number_of_vertices{
            deg[i] = self_deg[i];
            comp[i] = self_comp[i];
        }
        ZDDNode {
            deg: Some(RefCell::new(deg)),
            comp: Some(RefCell::new(comp)),
            sol: 0,
            zero_child: None,
            one_child: None,
            id: id,
        }
    }
    fn set_child(&mut self, node: Rc<RefCell<Self>>, child_num: usize) {
        if child_num == 0 {
            self.zero_child = Some(node);
        } else {
            self.one_child = Some(node);
        }
    }
    fn get_child(&self, child_num: i64) -> Rc<RefCell<Self>> {
        if child_num == 0 {
            self.zero_child.as_ref().unwrap().clone()
        } else {
            self.one_child.as_ref().unwrap().clone()
        }
    }
}

pub struct State {
    graph: std::rc::Rc<Graph>,
    s: usize,
    t: usize,
    frontier: Vec<Vec<usize>>,
}

impl State {
    pub fn new(graph: Graph, start: usize, end: usize) -> Self {
        let graph = std::rc::Rc::new(graph);
        State {
            s: start,
            t: end,
            graph: graph.clone(),
            frontier: State::compute_frontier(graph.clone()),
        }
    }
    fn compute_frontier(graph: std::rc::Rc<Graph>) -> Vec<Vec<usize>>{
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
                let mut v = frontier[i + 1].clone().into_iter().filter(|&i| i != src).collect::<Vec<_>>();
                frontier[i + 1].truncate(0);
                frontier[i + 1].append(&mut v);
            }
            if !State::find_element(graph.clone(), i, dst) {
                let mut v = frontier[i + 1].clone().into_iter().filter(|&i| i != dst).collect::<Vec<_>>();
                frontier[i + 1].truncate(0);
                frontier[i + 1].append(&mut v);
            }
        }
        frontier
    }
    fn find_element(graph: std::rc::Rc<Graph>, edge_number: usize, value: usize) -> bool {
        let edge_list = graph.get_edge_list();
        for i in edge_number + 1..edge_list.len() {
            if  value == edge_list[i].src || value == edge_list[i].dst {
                return true
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct ZDD {
    node_list_array: Vec<Vec<Rc<RefCell<ZDDNode>>>>,
}

impl ZDD {
    pub fn get_number_of_nodes(&self) -> usize {
        let mut num = 0;
        for i in 1..self.node_list_array.len() {
            num += self.node_list_array[i].len()
        }
        num + 2
    }
    pub fn get_number_of_solutions(&mut self) -> i64 {
        let mut i = self.node_list_array.len() - 1;
        while i > 0 {
            for j in 0..self.node_list_array[i].len() {
                let ij_node = self.node_list_array[i][j].clone();
                let lo_node_sol = ij_node.borrow().get_child(0).clone().borrow().sol;
                let hi_node_sol = ij_node.borrow().get_child(1).clone().borrow().sol;
                self.node_list_array[i][j].borrow_mut().sol = lo_node_sol + hi_node_sol;
            }
            i -= 1;
        }
        self.node_list_array[1][0].borrow().sol
    }
}

pub struct Frontier {
    total_zddnode_id: RefCell<usize>,
    zero_t: ZDDNode,
    one_t: ZDDNode,
}

impl Frontier {
    pub fn new() -> Self{
        let zero_t = ZDDNode {
            deg: None,
            comp: None,
            sol: 0,
            zero_child: None,
            one_child: None,
            id: 0,
        };
        let one_t = ZDDNode {
            deg: None,
            comp: None,
            sol: 1,
            zero_child: None,
            one_child: None,
            id: 1,
        };
        Self {
            total_zddnode_id: RefCell::new(1),
            zero_t: zero_t,
            one_t: one_t,
        }
    }
    fn get_zddnode_id(&self) -> usize {
        *self.total_zddnode_id.borrow_mut() += 1;
        let next_id = *self.total_zddnode_id.borrow();
        next_id
    }
    pub fn construct(&self, state: &State) -> ZDD {
        let edge_list = state.graph.get_edge_list();
        let mut N = vec![vec![]; edge_list.len() + 2];
        N[1].push(Rc::new(RefCell::new(ZDDNode::create_root_node(state.graph.get_number_of_vertices(),
                                                                    self.get_zddnode_id()))));

        for i in 1..=edge_list.len() {
            let mut n_i_1 = N[i + 1].clone();
            for j in 0..N[i].len() {
                let n_hat = N[i][j].clone();
                for x in 0..=1 {
                    let n_prime = {
                        let ref_n_hat = n_hat.borrow();
                        self.check_terminal(&ref_n_hat, i, x, state)
                    };
                    let n_prime = match n_prime {
                        None => {
                            let mut n_prime = n_hat.borrow().make_copy(state.graph.get_number_of_vertices(),
                                                                       *self.total_zddnode_id.borrow());
                            self.update_info(&n_prime, i, x, state);
                            let n_primeprime = self.find(&n_prime, &n_i_1, i, state);
                            let n_prime = match n_primeprime {
                                Some(v) => v,
                                None => {
                                    n_prime.set_next_id(self.get_zddnode_id());
                                    let new_prime = Rc::new(RefCell::new(n_prime));
                                    n_i_1.push(new_prime.clone());
                                    new_prime
                                }
                            };
                            Some(n_prime)
                        },
                        Some(v) => Some(Rc::new(RefCell::new(v.clone()))),
                    };
                    n_hat.borrow_mut().set_child(n_prime.unwrap(), x);
                }
            }
            N[i + 1] = n_i_1;
        }
        ZDD { node_list_array: N }
    }
    fn check_terminal(&self, n_hat: &ZDDNode,
                      i: usize, x: usize, state: &State) -> Option<&ZDDNode> {
        let edge = &state.graph.get_edge_list()[i - 1];
        let comp = n_hat.comp.as_ref().unwrap().borrow();
        if x == 1 {
            if comp[edge.src] == comp[edge.dst] {
                return Some(&self.zero_t);
            }
        }
        let n_prime = n_hat.make_copy(state.graph.get_number_of_vertices(),
                                          *self.total_zddnode_id.borrow());
        self.update_info(&n_prime, i, x, state);
        let ref_deg = &n_prime.deg.unwrap().into_inner();
        for y in 0..=1 {
            let u = match y {
                0 => edge.src,
                _ => edge.dst,
            };
            if (u == state.s || u == state.t) && ref_deg[u] > 1 {
                return Some(&self.zero_t);
            } else if  (u != state.s && u != state.t) && ref_deg[u] > 2 {
                return Some(&self.zero_t);
            }
        }
        for y in 0..=1 {
            let u = match y {
                0 => edge.src,
                _ => edge.dst,
            };
            if !state.frontier[i].contains(&u) {
                if (u == state.s || u == state.t) && ref_deg[u] != 1 {
                    return Some(&self.zero_t);
                } else if  (u != state.s && u != state.t)
                    && ref_deg[u] != 0
                    && ref_deg[u] != 2 {
                    return Some(&self.zero_t);
                }
            }
        }
        if i == state.graph.edge_list.len() {
            return Some(&self.one_t);
        }
        None
    }

    fn update_info(&self, n_hat: &ZDDNode, i: usize, x: usize, state: &State) {
        let edge = &state.graph.get_edge_list()[i - 1];
        let mut deg = n_hat.deg.as_ref().unwrap().borrow_mut();
        let mut comp = n_hat.comp.as_ref().unwrap().borrow_mut();
        for y in 0..=1 {
            let u = match y {
                0 => edge.src,
                _ => edge.dst,
            };
            if !state.frontier[i - 1].contains(&u) {
                deg[u] = 0;
                comp[u] = u;
            }
        }
        if x == 1 {
            deg[edge.src] += 1;
            deg[edge.dst] += 1;
            let (c_max, c_min) = {
                if comp[edge.src] > comp[edge.dst] {
                    (comp[edge.src], comp[edge.dst])
                } else {
                    (comp[edge.dst], comp[edge.src])
                }
            };
            for j in 0..state.frontier[i].len() {
                let u = state.frontier[i][j];
                if comp[u] == c_max {
                    comp[u] = c_min;
                }
            }
        }
    }
    fn find(&self,
            n_prime: &ZDDNode,
            n_i: &Vec<Rc<RefCell<ZDDNode>>>,
            i: usize, state: &State) -> Option<Rc<RefCell<ZDDNode>>> {
        for j in 0..n_i.len() {
            let n_primeprime = n_i[j].clone();
            if self.is_equivalent(&n_prime, n_primeprime.clone(), i, state) {
                return Some(n_primeprime.clone());
            }
        }
        None
    }
    fn is_equivalent(&self,
                     node1: &ZDDNode,
                     node2: Rc<RefCell<ZDDNode>>,
                     i: usize, state: &State) -> bool {
        let frontier = &state.frontier[i];
        let node2 = node2.borrow();
        let n1_deg = node1.deg.as_ref().unwrap().borrow();
        let n1_comp = node1.comp.as_ref().unwrap().borrow();
        let n2_deg = node2.deg.as_ref().unwrap().borrow();
        let n2_comp = node2.comp.as_ref().unwrap().borrow();
        for j in 0..frontier.len() {
            let v = frontier[j];
            if n1_deg[v] != n2_deg[v] {
                return false
            }
            if n1_comp[v] != n2_comp[v] {
                return false
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn zdd_create_root_node_fixtures(size: usize) -> (RefCell<Vec<usize>>,
                                                      RefCell<Vec<usize>>) {
        let comp = RefCell::new((1..=size).collect());
        let deg = RefCell::new(vec![0; size]);
        (comp, deg)
    }
    #[test]
    fn zddnode_create_root_node() {
        for i in 0..10 {
            let z = ZDDNode::create_root_node(i, 2);
            let (expected_comp, expected_deg) = zdd_create_root_node_fixtures(i);
            assert_eq!(z.comp.unwrap(), expected_comp);
            assert_eq!(z.deg.unwrap(), expected_deg);
        }
        for i in 10000..10010 {
            let z = ZDDNode::create_root_node(i, 2);
            let (expected_comp, expected_deg) = zdd_create_root_node_fixtures(i);
            assert_eq!(z.comp.unwrap(), expected_comp);
            assert_eq!(z.deg.unwrap(), expected_deg);
        }
        for i in 1000000..1000010 {
            let z = ZDDNode::create_root_node(i, 2);
            let (expected_comp, expected_deg) = zdd_create_root_node_fixtures(i);
            assert_eq!(z.comp.unwrap(), expected_comp);
            assert_eq!(z.deg.unwrap(), expected_deg);
        }
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
    fn zddnode_get_id() {
        let z1: ZDDNode = ZDDNode {
            deg: None,
            comp: None,
            sol: 0,
            zero_child: None,
            one_child: None,
            id: 0,
        };
        assert_eq!(z1.get_id(), 0);
    }
    #[test]
    fn zddnode_get_child() {
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
