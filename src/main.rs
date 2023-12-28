#[derive(Debug)]
struct GedNode<'g> {
    // level: u16,
    // tag: &'g str,
    // link: Option<&'g str>, // actually a letter followed by a number
    // value: Vec<&'g str>,
    value: &'g str,
    children: Vec<GedNode<'g>>,
}

type Line<'g> = &'g [(u16, &'g str)];

impl<'g> GedNode<'g> {
    fn parse(lines: Line<'g>) -> Option<(Line<'g>, Self)> {
        // Get the first line
        // Assume everything after is children
        let ((lvl, line), mut children) = lines.split_first()?;
        dbg!(lvl, line, children);

        // lines find next index where level (1st elem) = current
        // The rest are not children
        let mut rest = &[][..];
        if let Some(next_node_ind) = children.iter().position(|(l, _)| l == lvl) {
            dbg!(next_node_ind);
            let (cn, rs) = children.split_at(next_node_ind);
            children = cn;
            rest = rs;
        }
        dbg!(children, rest);

        // current element
        let mut cur_node = Self {
            // level: *lvl,
            value: line,
            children: vec![],
        };

        // 1: tag (if starts with @ then @..@ is a link)
        // *: data

        // children:
        while let Some((cn, child)) = Self::parse(children) {
            cur_node.children.push(child);
            children = cn;
        }

        dbg!(&cur_node);

        // return remainder and self
        Some((rest, cur_node))
    }
}

fn main() {
    let ged_data = &include_str!("../export-Forest.ged");

    let all_lines: Vec<(u16, _)> = ged_data
        .trim_matches('\u{feff}')
        .lines()
        .take(10)
        .filter_map(|l| {
            if let Some((lvl_digit, other)) = l.trim().split_once(' ') {
                if let Ok(lvl) = lvl_digit.parse() {
                    return Some((lvl, other));
                }
            }
            None
        })
        .collect();

    let mut lines = &all_lines[..];
    let mut nodes = Vec::new();

    while let Some((ls, node)) = GedNode::parse(lines) {
        nodes.push(node);
        lines = ls;
    }

    dbg!(nodes);
}
