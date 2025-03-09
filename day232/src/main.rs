use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str;
use std::time::Instant;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = Instant::now();
    let input_path = env::args().nth(1).expect("no input path");
    let input = fs::read(&input_path)?;
    println!("Read {input_path}.");

    let mut solution = 0;
    let nodes: HashMap<(u8, u8), HashSet<(u8, u8)>> = input
        .trim_ascii()
        .split(|v| *v == b'\n')
        .fold(HashMap::new(), |mut map, line| {
            let a = (line[0], line[1]);
            let b = (line[3], line[4]);

            map.entry(a).or_default().insert(b);
            map.entry(b).or_default().insert(a);

            map
        });

    let mut cliques: HashMap<(u8, u8), HashSet<(u8, u8)>> = HashMap::new();
    let mut maxc: HashSet<(u8, u8)> = HashSet::new();
    let mut i = 0;
    for (node, connected) in &nodes {
        // println!("{}{}: {}", node.0 as char, node.1 as char, connected.len());

        let mut c = HashSet::new();

        c.insert(*node);

        for n in connected {
            if c.is_subset(nodes.get(&n).unwrap()) {
                c.insert(*n);
            }
        }
        if c.len() == 13 {
            i += 1;
            println!("c{i}");
        }
        if c.len() > maxc.len() {
            maxc = c.clone();
        }
        cliques.insert(*node, c);
    }

    // for (node, clique) in cliques {
    //     println!("{}{}: {}", node.0 as char, node.1 as char, clique.len());
    // }
    //
    let mut x: Vec<(u8, u8)> = maxc.into_iter().collect();
    x.sort_unstable();
    for node in x {
        print!("{}{},", node.0 as char, node.1 as char);
    }
    println!();
    println!("Solution: {:#?} / Duration: {:.6?}", solution, t0.elapsed());
    Ok(())
}

// fn clique(set: &HashSet<(u8, u8)>, graph: &HashMap<(u8, u8), HashSet<(u8, u8)>>) -> bool {
//     for node in set {}

//     false
// }

/*
bron kerbosh
class Node(object):

    def __init__(self, name):
        self.name = name
        self.neighbors = []

    def __repr__(self):
        return self.name


A = Node('A')
B = Node('B')
C = Node('C')
D = Node('D')
E = Node('E')
F = Node('F')

A.neighbors = [B, C, E]
B.neighbors = [A, C, D, F]
C.neighbors = [A, B, D, F]
D.neighbors = [C, B, E, F]
E.neighbors = [A, D]
F.neighbors = [B, C, D]

all_nodes = [A, B, C, D, E, F]


def find_cliques(potential_clique=[], remaining_nodes=[], skip_nodes=[], depth=0):

    if len(remaining_nodes) == 0 and len(skip_nodes) == 0:
        print('This is a clique:', potential_clique)
        return 1

    found_cliques = 0
    for node in remaining_nodes:

        # Try adding the node to the current potential_clique to see if we can make it work.
        new_potential_clique = potential_clique + [node]
        new_remaining_nodes = [n for n in remaining_nodes if n in node.neighbors]
        new_skip_list = [n for n in skip_nodes if n in node.neighbors]
        found_cliques += find_cliques(new_potential_clique, new_remaining_nodes, new_skip_list, depth + 1)

        # We're done considering this node.  If there was a way to form a clique with it, we
        # already discovered its maximal clique in the recursive call above.  So, go ahead
        # and remove it from the list of remaining nodes and add it to the skip list.
        remaining_nodes.remove(node)
        skip_nodes.append(node)
    return found_cliques

total_cliques = find_cliques(remaining_nodes=all_nodes)
print('Total cliques found:', total_cliques)
*/

/*
k- clique
from itertools import combinations
import networkx as nx


def k_cliques(graph):
    # 2-cliques
    cliques = [{i, j} for i, j in graph.edges() if i != j]
    k = 2

    while cliques:
        # result
        yield k, cliques

        # merge k-cliques into (k+1)-cliques
        cliques_1 = set()
        for u, v in combinations(cliques, 2):
            w = u ^ v
            if len(w) == 2 and graph.has_edge(*w):
                cliques_1.add(tuple(u | w))

        # remove duplicates
        cliques = list(map(set, cliques_1))
        k += 1


def print_cliques(graph, size_k):
    for k, cliques in k_cliques(graph):
        if k == size_k:
            print('%d-cliques = %d, %s.' % (k, len(cliques), cliques))


nodes, edges = 6, 10
size_k = 3
graph = nx.Graph()
graph.add_nodes_from(range(nodes))
graph.add_edge(1, 2)
graph.add_edge(1, 3)
graph.add_edge(1, 5)
graph.add_edge(2, 3)
graph.add_edge(2, 4)
graph.add_edge(2, 6)
graph.add_edge(3, 4)
graph.add_edge(3, 6)
graph.add_edge(4, 5)
graph.add_edge(4, 6)

print_cliques(graph, size_k)
*/

/*
from collections import defaultdict
import random

def find_single_clique(graph):
    clique = []
    vertices = list(graph.keys())
    rand = random.randrange(0, len(vertices), 1)
    clique.append(vertices[rand])
    for v in vertices:
        if v in clique:
            continue
        isNext = True
        for u in clique:
            if u in graph[v]:
                continue
            else:
                isNext = False
                break
        if isNext:
            clique.append(v)

    return sorted(clique)

graph = dict()
graph['A'] = ['B', 'C', 'E']
graph['B'] = ['A', 'C', 'D', 'F']
graph['C'] = ['A', 'B', 'D', 'F']
graph['D'] = ['C', 'E', 'B', 'F']
graph['E'] = ['A', 'D']
graph['F'] = ['B', 'C', 'D']

clique = find_single_clique(graph)
print('A maximal clique in the graph is: ', clique)
*/
