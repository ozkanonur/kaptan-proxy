use config_compiler::config::{HeaderStruct, RoutesStruct};

pub struct RouteDependencies {
    /// Destionation address
    pub dest_addr: String,
    /// Request Headers
    pub req_headers: Option<Vec<HeaderStruct>>,
    /// Response Headers
    pub res_headers: Option<Vec<HeaderStruct>>,
}

impl RouteDependencies {
    fn new() -> Self {
        Self {
            dest_addr: String::new(),
            req_headers: None,
            res_headers: None,
        }
    }
}

pub struct RouterService {}

impl RouterService {
    /// Finds destination address and request & response headers
    /// depended on incoming route and returns it as instance of
    /// RouteDependencies.
    #[inline]
    pub fn get_dependencies(&mut self, req_uri: String, routes: &Vec<RoutesStruct>) -> RouteDependencies {
        let mut rd = RouteDependencies::new();

        let mut index = routes.iter().position(|r| {
            r.inbound_route == req_uri || r.inbound_route.to_owned() + "/" == req_uri
        });

        if index.is_some() {
            rd.dest_addr = routes[index.unwrap()].dest_addr.to_string();
        } else {
            index = routes.iter().position(|r| r.inbound_route == "/");

            if index.is_some() {
                rd.dest_addr = routes[index.unwrap()].dest_addr.to_string();
                rd.dest_addr.push_str(&req_uri);
            }
        }

        if rd.dest_addr.chars().last() != Some('/') {
            rd.dest_addr.push('/');
        }

        let index = index.unwrap();
        rd.req_headers = routes[index].res_headers.clone();
        rd.res_headers = routes[index].res_headers.clone();

        rd
    }
}
