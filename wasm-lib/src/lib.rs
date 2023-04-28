use base64;
use std::collections::HashMap;
use std::collections::HashSet;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn fcc(base64_string: &str) -> String {
    let pdb = decode_base64(base64_string);

    let input_pdbs = parse_input(&pdb);

    // Calculate the contacts for each PDB file and add them to the HashMap
    let mut models = HashMap::new();

    // for (i, pdb) in input_pdbs.iter().enumerate() {
    for (model_index, pdb) in input_pdbs.into_iter() {
        let contacts = calculate_contacts(&pdb);

        // let model_index = i as i32 + 1;
        models.insert(model_index, contacts.clone());
    }
    let cluster_mat = calculate_pairwise_fcc(models);

    let elements = create_elements(cluster_mat);

    let clusters = cluster(elements);

    output(clusters)
}

fn decode_base64(base64_string: &str) -> String {
    // let decoded_bytes = base64::decode(base64_string).unwrap();
    let input = base64_string.replace("data:application/octet-stream;base64,", "");
    let decoded_bytes = match base64::decode(input) {
        Ok(v) => String::from_utf8(v).unwrap(),
        Err(_) => "".to_string(),
    };
    decoded_bytes
}

struct Atom {
    x: f64,
    y: f64,
    z: f64,
}

struct Residue {
    chainid: String,
    resnum: i32,
    atoms: Vec<Atom>,
}

struct Element {
    name: String,
    neighbors: HashSet<String>,
}

struct Cluster {
    position: i32, // position of the cluster in the cluster matrix
    center: String,
    members: Vec<i32>,
}

// fn main() {
//     // let args: Vec<String> = env::args().collect();

//     // Get the list of PDB files to be processed
//     // let input = "pdb.list".to_owned();
//     let input = "ensemble.pdb".to_owned();
//     // let pdblist = parse_input(&args[1]);
//     // let pdblist = parse_input(&input);
//     let input_pdbs = parse_input(&input);

//     // Calculate the contacts for each PDB file and add them to the HashMap
//     let mut models = HashMap::new();

//     // for (i, pdb) in input_pdbs.iter().enumerate() {
//     for (model_index, pdb) in input_pdbs.into_iter() {
//         let contacts = calculate_contacts(&pdb);

//         // let model_index = i as i32 + 1;
//         models.insert(model_index, contacts.clone());
//     }
//     let cluster_mat = calculate_pairwise_fcc(models);

//     let elements = create_elements(cluster_mat);

//     let clusters = cluster(elements);

//     output(clusters)
// }

// Calculate the contacts for a given PDB file
fn calculate_contacts(pdb: &Vec<String>) -> HashSet<String> {
    let mut residues = HashMap::<(String, i32), Residue>::new();

    for line in pdb {
        // If the line is a record of an atom
        if line.starts_with("ATOM") {
            // Ignore Hydrogen atoms
            let element: String = line[76..78].to_string().trim().to_string();
            if element == "H" {
                continue;
            }

            // Get the atom's coordinates
            let x: f64 = line[30..38].trim().parse::<f64>().unwrap();
            let y: f64 = line[38..46].trim().parse::<f64>().unwrap();
            let z: f64 = line[46..54].trim().parse::<f64>().unwrap();

            // Get the chainID
            let chainid: String = line[21..22].to_string().trim().to_string();

            // Get the resnum
            let resnum: i32 = line[22..26].trim().parse::<i32>().unwrap();

            //
            let res = residues
                .entry((chainid.clone(), resnum.clone()))
                .or_insert(Residue {
                    chainid,
                    resnum,
                    atoms: vec![],
                });
            res.atoms.push(Atom { x, y, z });
        }
    }

    let mut output = HashSet::<String>::new();

    for res_a in residues.values() {
        for res_b in residues.values() {
            // Only calculate for residues in different chains
            if res_a.chainid == res_b.chainid {
                continue;
            }

            // Calculate the euclidean distance between the atoms
            for atom_a in res_a.atoms.iter() {
                for atom_b in res_b.atoms.iter() {
                    let dist: f64 = ((atom_a.x - atom_b.x).powi(2)
                        + (atom_a.y - atom_b.y).powi(2)
                        + (atom_a.z - atom_b.z).powi(2))
                    .sqrt();
                    if dist <= 5.0 {
                        // output.push(contact);
                        let contact: String = format!(
                            "{} {} {} {}",
                            res_a.chainid, res_a.resnum, res_b.chainid, res_b.resnum,
                        );
                        output.insert(contact);
                    }
                }
            }
        }
    }

    output
}

// Loop over a vector of vector of residues and calculate the contacts
// fn calculate_contacts(models: Vec<Vec<Res>)

// Calculate the frequency of common contact between two sets of vectors
fn calculate_fcc(x: &HashSet<String>, y: &HashSet<String>) -> (f64, f64) {
    // Get how many elements of X are in Y
    let cc = x.intersection(&y).count() as i32;
    let cc_v = y.intersection(&x).count() as i32;

    let fcc = cc as f64 / x.len() as f64;
    let fcc_v = cc_v as f64 / y.len() as f64;

    (fcc, fcc_v)
}

// Calculate the frequency of common contacts between each pair of models
fn calculate_pairwise_fcc(
    models: HashMap<String, HashSet<String>>,
) -> Vec<(String, String, f64, f64)> {
    let mut cluster_mat = Vec::<(String, String, f64, f64)>::new();

    // Calculate the frequency of common contacts between each pair of models
    for (model_one, contacts_one) in &models {
        for (model_two, contacts_two) in &models {
            let (fcc, fcc_v) = calculate_fcc(contacts_one, contacts_two);
            cluster_mat.push((model_one.clone(), model_two.clone(), fcc, fcc_v));
        }
    }
    cluster_mat
}

// Read the input from a pdb.list file
fn parse_input(pdb_list: &String) -> HashMap<String, Vec<String>> {
    // let file = File::open(pdb_list).expect("Cannot open file");

    let mut models = HashMap::<String, Vec<String>>::new();

    // let mut pdblist = Vec::<String>::new();
    let mut model_index = "0".to_string();
    // for line in BufReader::new(file).lines() {
    for line in pdb_list.lines() {
        // let line = line.unwrap();

        // If the line is the MODEL record, create a new entry in the HashMap
        if line.starts_with("MODEL") {
            model_index = line[10..14].trim().to_string();
            models.insert(model_index.clone(), Vec::<String>::new());
            continue;
        }

        // If the line is the ATOM record, add it to the vector of the current model
        if line.starts_with("ATOM") {
            let model = models.get_mut(&model_index).unwrap();
            model.push(line.to_string());
            continue;
        }
    }
    models
}

// Cluster
fn cluster(mut elements: HashMap<String, Element>) -> Vec<Cluster> {
    let mut used = Vec::<String>::new();

    let mut clusters = Vec::<Cluster>::new();
    loop {
        // Get the clusterable elements, the keys from elements that are not in used
        let mut clusterable = Vec::from_iter(elements.keys());
        clusterable.retain(|x| !used.contains(x));

        if clusterable.len() == 0 {
            break;
        }

        // Get the model with the most neighbors
        let mut population_map = HashMap::<i32, i32>::new();
        for model in clusterable {
            population_map.insert(
                model.parse().unwrap(),
                elements.get(model).unwrap().neighbors.len() as i32,
            );
        }
        // Sort the population map, so that if there are multiple elements with the same
        //  number of neighbors, the one with the highest number is selected
        let mut sorted: Vec<_> = population_map.iter().collect();
        sorted.sort_by_key(|k| k.0);

        // Pick the cluster center (the model with the most neighbors)
        let cluster_center = sorted
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .0
            .to_string();

        // Get the neighbours of the cluster center
        let neighbours = elements.get(&cluster_center).unwrap().neighbors.clone();

        // If the member is not in the used list, add it to the cluster
        //  For compatibility purposes, create a new vector with the members and sort it before printing
        let mut cluster_members = Vec::<i32>::new();
        for member in &neighbours {
            if used.contains(member) {
                continue;
            } else {
                // Ignore the seed
                if member == &cluster_center {
                    continue;
                }

                cluster_members.push(member.clone().parse().unwrap());

                // Add to the used list
                used.push(member.to_string());
            }
        }

        // Add the seed to the used list
        used.push(cluster_center.clone());

        // Sort cluster_members ascending
        cluster_members.sort();

        // Remove the seed from the neighbor list
        elements.remove(&cluster_center);

        clusters.push(Cluster {
            // number: cluster_number,
            position: 0,
            center: cluster_center,
            members: cluster_members,
        });
    }
    clusters
}

// Create the elements
fn create_elements(cluster_mat: Vec<(String, String, f64, f64)>) -> HashMap<String, Element> {
    let cutoff = 0.60;
    let strictness = 0.75;
    let mut elements = HashMap::<String, Element>::new();
    // Create the elements
    for (model_one, model_two, fcc, fcc_v) in cluster_mat {
        // Check if model_one is in the r HashMap
        let element_one = elements.entry(model_one.clone()).or_insert(Element {
            name: model_one.clone(),
            neighbors: HashSet::new(),
        });
        element_one.name = model_one.clone();

        if fcc >= cutoff && fcc_v >= cutoff * strictness {
            element_one.neighbors.insert(model_two.clone());
        }

        // Check if model_two is in the m HashMap
        let element_two = elements.entry(model_two.clone()).or_insert(Element {
            name: model_two.clone(),
            // cluster: 0,
            neighbors: HashSet::new(),
        });
        element_two.name = model_two.clone();

        // Assign Neighbors
        if fcc_v >= cutoff && fcc >= cutoff * strictness {
            element_two.neighbors.insert(model_one.clone());
        }
    }

    elements
}

// Prepares the output
fn output(mut c: Vec<Cluster>) -> String {
    // Sort the clusters by the number of members
    c.sort_by(|a, b| b.members.len().cmp(&a.members.len()));

    // Set the position of each cluster
    for (i, cluster) in c.iter_mut().enumerate() {
        cluster.position = i as i32 + 1;
    }

    // Print the clusters
    let mut output = String::new();
    for cluster in c {
        if cluster.members.len() < 3 {
            continue;
        }

        // Format the members
        let mut members = String::new();
        for member in cluster.members {
            members.push_str(&member.to_string());
            members.push_str(" ");
        }
        // println!(
        //     "Cluster {} -> {} {}",
        //     cluster.position, cluster.center, members
        // );
        output.push_str(&format!(
            "Cluster {} -> {} {}\n",
            cluster.position, cluster.center, members
        ));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode() {
        let v = "SGVsbG8gV29ybGQh";
        let result = decode_base64(v);
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn invalid_decode() {
        let v = "data: base64";
        let result = decode_base64(v);
        assert_eq!(result, "")
    }

    #[test]
    fn remove_header_and_decode() {
        let v = "data:application/octet-stream;base64,SGVsbG8gV29ybGQh";
        let result = decode_base64(v);
        assert_eq!(result, "Hello World!");
    }
}
