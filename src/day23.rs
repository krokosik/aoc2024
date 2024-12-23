use std::collections::HashSet;

use itertools::Itertools;

type Graph<'a> = (HashSet<&'a str>, HashSet<(&'a str, &'a str)>);

fn input_generator(input: &str) -> Graph {
    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();

    input.lines().for_each(|line| {
        let mut duo = line.split("-");
        let node1 = duo.next().unwrap();
        let node2 = duo.next().unwrap();
        nodes.insert(node1);
        nodes.insert(node2);
        edges.insert((node1, node2));
        edges.insert((node2, node1));
    });

    (nodes, edges)
}

fn find_triplets<'a>(lan_graph: &'a Graph<'a>) -> Vec<[&'a str; 3]> {
    let (nodes, edges) = lan_graph;
    let mut triplets = Vec::new();

    for &(node1, node2) in edges {
        for &node3 in nodes {
            if edges.contains(&(node1, node3)) && edges.contains(&(node3, node2)) {
                triplets.push([node1, node2, node3]);
            }
        }
    }

    triplets
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    let lan_graph = input_generator(input);
    let triplets = find_triplets(&lan_graph);

    // for triplet in triplets.iter() {
    //     println!("{}", triplet);
    // }

    triplets
        .iter()
        .filter(|triplet| triplet.iter().any(|n| n.starts_with(&"t")))
        .count()
        / 6
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 7);
    }
}
