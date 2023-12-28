#[derive(Debug)]
struct GedNode<'g> {
    // level: u16,
    tag: &'g str,
    // link: Option<&'g str>, // actually a letter followed by a number
    // value: Vec<&'g str>,
    value: Option<&'g str>,
    children: Vec<GedNode<'g>>,
}

type Line<'g> = &'g [(u16, &'g str)];

impl<'g> GedNode<'g> {
    fn parse(lines: Line<'g>) -> Option<(Line<'g>, Self)> {
        // Get the first line
        // Assume everything after is children
        let ((lvl, line), mut children) = lines.split_first()?;
        // dbg!(lvl, line, children);

        // lines find next index where level (1st elem) = current
        // The rest are not children
        let mut rest = &[][..];
        if let Some(next_node_ind) = children.iter().position(|(l, _)| l == lvl) {
            // dbg!(next_node_ind);
            let (cn, rs) = children.split_at(next_node_ind);
            children = cn;
            rest = rs;
        }
        // dbg!(children, rest);

        // current element
        let split: Vec<_> = line.splitn(2, ' ').collect();
        let mut cur_node = Self {
            // level: *lvl,
            tag: split[0],
            value: split.get(1).copied(),
            children: vec![],
        };

        // 1: tag (if starts with @ then @..@ is a link)
        // *: data

        // children:
        while let Some((cn, child)) = Self::parse(children) {
            cur_node.children.push(child);
            children = cn;
        }

        // dbg!(&cur_node);

        // return remainder and self
        Some((rest, cur_node))
    }
}

fn main() {
    let ged_data = &include_str!("../export-Forest.ged");

    let all_lines: Vec<(u16, _)> = ged_data
        .trim_matches('\u{feff}')
        .lines()
        // .take(43)
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

    let mut birthdays: Vec<_> = nodes
        .into_iter()
        .filter_map(|node| {
            let birt_node = node.children.iter().find(|n| n.tag == "BIRT")?;
            let name = node
                .children
                .iter()
                .find(|n| n.tag == "NAME")?
                .value?
                .replace('/', "");
            let birthday = birt_node.children.iter().find(|n| n.tag == "DATE")?.value?;

            let bd_parts: Vec<_> = birthday.split(' ').take(2).collect();
            let day = bd_parts[0].parse::<u16>().ok()?;
            let month = bd_parts.get(1)?;

            Some((day, *month, name))
        })
        .collect();

    birthdays.sort();

    let months: Vec<_> = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]
    .into_iter()
    .map(|m| (m, vec![]))
    .collect();

    let grouped_birthdays = birthdays
        .into_iter()
        .fold(months, |mut months, (day, month, name)| {
            let (_, grp) = months
                .iter_mut()
                .find(|(m, _)| &m.to_uppercase()[0..3] == month)
                .expect("no month-group found");

            grp.push((day, name));

            months
        });

    for (month, grp) in grouped_birthdays {
        println!("{month}");
        for (day, name) in grp {
            println!("{day}: {name}");
        }
        println!();
    }
}
