use yaml_rust;

pub struct EnvParser {}

impl EnvParser {
    pub fn parse_node_id(str: &str) -> Option<i64> {
        let docs = yaml_rust::YamlLoader::load_from_str(str).unwrap();
        let node_id = &docs[0]["node_id"];

        node_id.as_i64()
    }

    pub fn parse_next_node(str: &str) -> Option<String> {
        let docs = yaml_rust::YamlLoader::load_from_str(str).unwrap();
        let next_node = &docs[0]["next_node"];

        next_node.as_str().map(|str_ref| str_ref.to_string())
    }
}
