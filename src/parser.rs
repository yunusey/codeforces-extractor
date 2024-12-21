use tl;

#[derive(Debug)]
pub struct Problem {
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

/// Parses lines for "input" content in problem statement
fn parse_input(problem_node: &tl::Node, parser: &tl::Parser) -> Vec<String> {
    problem_node
        .children()
        .unwrap()
        .all(parser)
        .iter()
        .map(|inner_node: &tl::Node| {
            let Some(tag) = inner_node.as_tag() else {
                return None;
            };
            let attributes = tag.attributes();
            let class = attributes.get("class");
            if class.is_none() || class.unwrap() != Some(&tl::Bytes::from("input")) {
                return None;
            }

            // at this point, the node is an input node!
            let input = inner_node
                .children()
                .unwrap()
                .all(parser)
                .iter()
                .map(|element| {
                    if let Some(tag) = element.as_tag() {
                        let attributes = tag.attributes();
                        if let Some(class_name) = attributes.get("class") {
                            if class_name
                                .unwrap()
                                .as_bytes()
                                .starts_with(b"test-example-line")
                            {
                                let inner_text = element.inner_text(parser);
                                return format!("{}\n", inner_text.trim().to_string());
                            }
                        }
                        return "".to_string();
                    } else {
                        return "".to_string();
                    }
                })
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
                .join("");

            return Some(input);
        })
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect()
}

/// Parses lines for "output" content in problem statement
fn parse_output(problem_node: &tl::Node, parser: &tl::Parser) -> Vec<String> {
    problem_node
        .children()
        .unwrap()
        .all(parser)
        .iter()
        .map(|inner_node: &tl::Node| {
            let Some(tag) = inner_node.as_tag() else {
                return None;
            };
            let attributes = tag.attributes();
            let class = attributes.get("class");
            if class.is_none() || class.unwrap() != Some(&tl::Bytes::from("output")) {
                return None;
            }

            // at this point, the node is an output node!
            let output = inner_node
                .find_node(parser, &mut |inner_node: &tl::Node| {
                    if let Some(tag) = inner_node.as_tag() {
                        if tag.name() == "pre" {
                            return true;
                        }
                    }
                    return false;
                })
                .unwrap()
                .get(parser)
                .unwrap()
                .inner_text(parser)
                .to_string();
            Some(output)
        })
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect()
}

pub fn parse_content(content: &str) -> Vec<Problem> {
    let document = tl::parse(content, tl::ParserOptions::default()).unwrap();
    let parser = document.parser();
    document
        .get_elements_by_class_name("problem-statement")
        .map(&mut |node: tl::NodeHandle| {
            let problem_node = node.get(parser).unwrap();
            let header = problem_node
                .children()
                .unwrap()
                .top()
                .get(0)
                .unwrap()
                .get(parser)
                .unwrap();
            let title = header
                .children()
                .unwrap()
                .top()
                .get(0)
                .unwrap()
                .get(parser)
                .unwrap();
            let problem_name = title.inner_text(parser);
            let inputs = parse_input(&problem_node, &parser);
            let outputs = parse_output(&problem_node, &parser);
            let problem = Problem {
                name: problem_name.to_string(),
                inputs,
                outputs,
            };
            return problem;
        })
        .collect()
}
