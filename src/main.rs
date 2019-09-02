use serde::Serialize;
use clap::{App, Arg, ArgMatches};
use rand::seq::SliceRandom;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};

fn get_arguments() -> ArgMatches<'static> {
    let matches = App::new("MIS Rand")
        .version("0.1a")
        .author("D. James Skelton <d.j.skelton1@gmail.com>")
        .about("Carries out multiple iterations of generating random MIS and returns the best")
        .arg(
            Arg::with_name("graph")
                .short("g")
                .long("graph")
                .help("The graph file (as edge list)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("no_iterations")
                .short("n")
                .long("no_iterations")
                .help("Number of iterations to run")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("small")
                .short("s")
                .long("small")
                .help("Prefer smaller maximal independent sets over larger ones"),
        )
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .help("Set the number of threads to use (default 0 uses all threads)")
                .takes_value(true)
        )
        .get_matches();

    matches
}

fn parse_edgelist(infile: &str) -> HashMap<String, Vec<String>> {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

    let csv = BufReader::new(File::open(infile).unwrap());

    for item in csv.lines() {
        let line = item.unwrap();
        let split_line: Vec<&str> = line
            .split_whitespace()
            .map(|i: &str| i.trim())
            .collect::<Vec<&str>>();

        let a = split_line.get(0).unwrap().to_string();
        let b = split_line.get(1).unwrap().to_string();

        let a_neighbors = adjacency.entry(a.clone()).or_insert_with(|| vec![]);
        a_neighbors.push(b.clone());

        let b_neighbors = adjacency.entry(b.clone()).or_insert_with(|| vec![]);
        b_neighbors.push(a.clone());
    }

    adjacency
}

fn run(adjacency: &HashMap<String, Vec<String>>) -> HashSet<&String> {
    // Make a hash set to store keys that are covered.
    let mut covered = HashSet::new();
    let mut selection = HashSet::new();

    // Shuffle the keys.
    let mut rng = rand::thread_rng();
    let mut keys = adjacency.keys().collect::<Vec<_>>();
    keys.shuffle(&mut rng);

    let mut iter_keys = keys.iter();

    // Find each time in the shuffled list that there is not covered.
    while let Some(i) = iter_keys.find(|i| !covered.contains(*i)) {
        selection.insert(*i);
        covered.insert(*i);
        covered.extend(adjacency.get(*i).unwrap());
    }

    selection
}

#[derive(Serialize)]
struct Best<'a> {
    size: usize,
    members: HashSet<&'a String>,
}

fn main() {
    // Parse the command line arguments
    let matches = get_arguments();

    // Parse an edge to make a HashMap of {Node : Neighbours}
    let edgelist_file = matches.value_of("graph").unwrap();
    let edges = parse_edgelist(edgelist_file);

    // Get the number of iterations declared (or default to 10,000)
    let n = matches
        .value_of("no_iterations")
        .unwrap_or("10000")
        .parse::<u32>()
        .unwrap();

    // Check whether to aim for bigger or smaller maximal independent sets.
    let prefer_smaller = matches.is_present("small");

    // Check whether the number of threads to use has been set by the user.
    let num_threads = matches.value_of("threads").unwrap_or("0")
        .parse::<usize>().unwrap();

    if num_threads != 0 {
        ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
    }

    // Get the best set.
    let best = match prefer_smaller {
        false => {
            let mut best: Best = Best {
                size: 0,
                members: HashSet::new(),
            };
            let guarded_best = Arc::new(Mutex::new(&mut best));

            (0u32..n).into_par_iter().for_each(|_| {
                let a = run(&edges);
                let mut best = guarded_best.lock().unwrap();

                if a.len() > best.size {
                    best.members.drain();
                    best.members.extend(a);
                    best.size = best.members.len();
                }
            });
            best
        }
        true => {
            let mut best: Best = Best {
                size: edges.len(),
                members: HashSet::new(),
            };
            let guarded_best = Arc::new(Mutex::new(&mut best));

            (0u32..n).into_par_iter().for_each(|_| {
                let a = run(&edges);
                let mut best = guarded_best.lock().unwrap();

                if a.len() < best.size {
                    best.members.drain();
                    best.members.extend(a);
                    best.size = best.members.len();
                }
            });
            best
        }
    };

    println!("{}", serde_json::to_string(&best).unwrap());
}
