pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                origin: String,
                destination: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(origin: &str, destination: &str) -> Self {
                    Edge {
                        origin: origin.to_string(),
                        destination: destination.to_string(),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    let mut hash_attrs: HashMap<String, String> = HashMap::new();
                    for (key, val) in attrs {
                        hash_attrs.insert(key.to_string(), val.to_string());
                    }

                    self.attrs = hash_attrs;
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    let mut hash_attrs: HashMap<String, String> = HashMap::new();
                    for (key, val) in attrs {
                        hash_attrs.insert(key.to_string(), val.to_string());
                    }

                    self.attrs = hash_attrs;
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }
            }            
        }
    }

    pub struct Graph {
        pub name: String,
        pub nodes: Vec<crate::graph::graph_items::node::Node>,
        pub edges: Vec<crate::graph::graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {        
        pub fn new() -> Self {
            Graph {
                name: String::new(),
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[crate::graph::graph_items::node::Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[crate::graph::graph_items::edge::Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            let mut hashed_attrs = HashMap::new();
            for (key, val) in attrs {
                hashed_attrs.insert(key.to_string(), val.to_string());
            }

            self.attrs = hashed_attrs;
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&crate::graph::graph_items::node::Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
    }
}
