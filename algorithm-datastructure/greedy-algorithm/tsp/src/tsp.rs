use std::collections::HashSet;
use std::cmp::Ordering;

use super::error::IterationError;

pub fn tsp(data: Vec<(usize, f64, f64)>) -> Result<i32, Box<dyn std::error::Error>> {
    // set explored as new hash
    // set head as first node and mark as explored
    // set tour = 0
    let n = data.len();
    let mut explored = HashSet::<usize>::new();
    let mut tour = 0.0f64;
    let mut head = &data[0];
    explored.insert(head.0);

    // while explored.len() < n
    while explored.len() < n {
        // next = min( distance from current head )
        let next = &data.iter()
            .filter(|&node| !explored.contains(&node.0))
            .min_by(|&a, &b| -> Ordering {
                // must apply sqrt, comparison will result in wrongly without it
                let dist_a = (a.1 - head.1).powi(2) + (a.2 - head.2).powi(2);
                let dist_b = (b.1 - head.1).powi(2) + (b.2 - head.2).powi(2);
                let dist_cmp = dist_a.partial_cmp(&dist_b);
                if dist_cmp == Some(Ordering::Equal) {
                    return a.0.partial_cmp(&b.0).unwrap_or(Ordering::Less)
                } else {
                    return dist_cmp.unwrap_or(Ordering::Equal)
                }
            })
            .ok_or(IterationError)?;
        // add tour distance
        tour += ((next.1 - head.1).powi(2) + (next.2 - head.2).powi(2)).sqrt();
        // mark next as explored
        explored.insert(next.0);
        // head = next
        head = next;
    }


    // add tour from head to 1st node
    tour += ((data[0].1 - head.1).powi(2) + (data[0].2 - head.2).powi(2)).sqrt();
    // retrun total distance as integer
    Ok(tour.floor() as i32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_nodes() {
        let data = vec![(1, 0.0, 0.0), (2, 1.0, 0.0)];
        assert_eq!(tsp(data).unwrap(), 2);
    }

    #[test]
    fn three_nodes() {
        let data = vec![(1, 0.0, 0.0), (2, 10.0, 0.0), (3, 10.0, 10.0)];
        assert_eq!(tsp(data).unwrap(), 34);
    }

    #[test]
    fn four_nodes() {
        let data = vec![(1, 0.0, 0.0), (2, 10.0, 0.0), (3, 0.0, 11.0), (4, 10.0, -9.0)];
        assert_eq!(tsp(data).unwrap(), 52);
    }

    #[test]
    fn six_nodes() {
        let data = vec![(1, 2.0, 1.0), (2, 4.0, 0.0), (3, 2.0, 0.0), (4, 0.0, 0.0), (5, 4.0, 3.0), (6, 0.0, 3.0)];
        assert_eq!(tsp(data).unwrap(), 15);
    }

    #[test]
    fn pick_node_of_smaller_index() {
        let data = vec![(1, 0.0, 0.0), (2, 0.0, 10.0), (3, 10.0, 0.0), (4, 0.0, -10.0)];
        assert_eq!(tsp(data).unwrap(), 48);
    }

    #[test]
    fn pick_node_of_smaller_index2() {
        let data = vec![(1, 17383.3333, 97716.6667), (2, 17350.0, 97733.3333), (3, 17366.6667, 97750.0), (4, 17340.0, 97733.3333)];
        assert_eq!(tsp(data).unwrap(), 115);
    }

    #[test]
    fn pick_node_of_smaller_index3() {
        let data = vec![(1, 0.0, 1.0), (2, 0.0, 1.5), (3, 1.0, 1.5), (4, -1.0, 1.5), (5, 1.0, 2.5), (6, 0.0, 2.5)];
        assert_eq!(tsp(data).unwrap(), 6);
    }
}

