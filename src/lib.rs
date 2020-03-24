// ref: https://github.com/junkawahara/frontier-basic

struct Edge {
    src: i64,
    dst: i64,
}

struct Graph {
    number_of_vertices: i64,
    edge_list: Vec<Edge>
}

impl Graph {
    fn get_number_of_vertices(self) -> i64 {
        self.number_of_vertices
    }
    fn get_edge_list(self) -> Vec<Edge> {
        self.edge_list
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

#[derive(Debug, Default)]
struct ZDDNode {
    deg: Option<Vec<i64>>,
    comp: Option<Vec<i64>>,
    sol: i64,
    zero_child: Option<Box<ZDDNode>>,
    one_child: Option<Box<ZDDNode>>,
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
            sol: 0,
            zero_child: None,
            one_child: None,
            id: 1,
    };
    fn set_next_id() -> i64;
    fn create_root_node(number_of_vertices: i64) -> Self;
    fn get_id(self) -> i64;
    fn make_copy(self, number_of_vertices: i64) -> Self;
    fn set_child(&mut self, node: Self, child_num: i64);
    fn get_child(self, child_num: i64) -> Box<Self>;
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
            self.zero_child = Some(Box::new(node));
        } else {
            self.one_child = Some(Box::new(node));
        }
    }
    fn get_child(self, child_num: i64) -> Box<Self> {
        if child_num == 0 {
            self.zero_child.unwrap()
        } else {
            self.one_child.unwrap()
        }
    }
}

struct State {
    graph: std::rc::Rc<Graph>,
    s: i64,
    t: i64,
    F: Vec<Vec<i64>>,
}

impl State {
    fn new(graph: Graph, start: i64,  end: i64) -> Self {
        State {
            s: start,
            t: end,
            graph: std::rc::Rc::new(graph),
            F: State::compute_frontier(std::rc::Rc::new(graph)),
        }
    }
    fn compute_frontier(graph: std::rc::Rc<Graph>) -> Vec<Vec<i64>>{
        let edge_list: Vec<Edge> = graph.get_edge_list();
        let mut F = vec![vec![]; edge_list.len() + 1];

        for i in 0..edge_list.len() {
            for j in 0..F[i].len() {
                F[i + 1].push
            }
        }


        vec![vec![0, 2, 3]]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zdd_create_root_node_3() {
        let z = ZDDNode::create_root_node(3);
        assert_eq!(z.comp.unwrap(), vec![1, 2]);
        assert_eq!(z.deg.unwrap(), vec![0, 0]);
    }
    #[test]
    fn test_zdd_create_root_node_1() {
        let z = ZDDNode::create_root_node(1);
        assert_eq!(z.comp.unwrap(), vec![]);
        assert_eq!(z.deg.unwrap(), vec![]);
    }
    #[test]
    fn test_zdd_create_root_node_2() {
        let z = ZDDNode::create_root_node(2);
        assert_eq!(z.comp.unwrap(), vec![1]);
        assert_eq!(z.deg.unwrap(), vec![0]);
    }
    #[test]
    fn test_zdd_create_root_node_edge_case() {
        let z = ZDDNode::create_root_node(0);
        assert_eq!(z.comp.unwrap(), vec![]);
        assert_eq!(z.deg.unwrap(), vec![]);
    }
}
