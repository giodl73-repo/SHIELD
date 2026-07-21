use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::graph::{NodeIndex, UnGraph};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FacilityRole {
    Hospital,
    Clinic,
    Specialty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DemandBasis {
    Surge,
    Baseline,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Facility {
    pub id: String,
    pub name: String,
    pub role: FacilityRole,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Referral {
    pub id: String,
    pub capacity_beds: f64,
    pub basis: DemandBasis,
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("duplicate facility id: {0}")]
    DuplicateFacility(String),
    #[error("unknown facility id: {0}")]
    UnknownFacility(String),
    #[error("non-positive capacity_beds: {0}")]
    NonPositiveCapacity(f64),
}

pub struct Network {
    graph: UnGraph<Facility, Referral>,
    index: HashMap<String, NodeIndex>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            graph: UnGraph::new_undirected(),
            index: HashMap::new(),
        }
    }

    pub fn add_facility(&mut self, facility: Facility) -> Result<(), NetworkError> {
        if self.index.contains_key(&facility.id) {
            return Err(NetworkError::DuplicateFacility(facility.id));
        }
        let id = facility.id.clone();
        let idx = self.graph.add_node(facility);
        self.index.insert(id, idx);
        Ok(())
    }

    pub fn add_referral(
        &mut self,
        from_id: &str,
        to_id: &str,
        referral: Referral,
    ) -> Result<(), NetworkError> {
        if referral.capacity_beds <= 0.0 {
            return Err(NetworkError::NonPositiveCapacity(referral.capacity_beds));
        }
        let from = match self.index.get(from_id) {
            Some(&i) => i,
            None => return Err(NetworkError::UnknownFacility(from_id.to_string())),
        };
        let to = match self.index.get(to_id) {
            Some(&i) => i,
            None => return Err(NetworkError::UnknownFacility(to_id.to_string())),
        };
        self.graph.add_edge(from, to, referral);
        Ok(())
    }

    pub fn facility_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn referral_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn degree(&self, id: &str) -> Option<usize> {
        self.index.get(id).map(|&idx| self.graph.edges(idx).count())
    }

    pub fn is_connected(&self, a: &str, b: &str) -> bool {
        match (self.index.get(a), self.index.get(b)) {
            (Some(&x), Some(&y)) => self.find_path(x, y).is_some(),
            _ => false,
        }
    }

    pub fn has_diverse_path(&self, a: &str, b: &str) -> bool {
        let (start, goal) = match (self.index.get(a), self.index.get(b)) {
            (Some(&x), Some(&y)) => (x, y),
            _ => return false,
        };
        if start == goal {
            return false;
        }
        let path = match self.find_path(start, goal) {
            Some(p) => p,
            None => return false,
        };
        let intermediates: HashSet<NodeIndex> = path
            .iter()
            .copied()
            .filter(|&n| n != start && n != goal)
            .collect();
        self.find_path_avoiding(start, goal, &intermediates)
            .is_some()
    }

    pub fn incident_capacity_beds(&self, id: &str) -> f64 {
        let idx = match self.index.get(id) {
            Some(&i) => i,
            None => return 0.0,
        };
        let mut total = 0.0;
        for edge in self.graph.edge_indices() {
            if let Some((a, b)) = self.graph.edge_endpoints(edge) {
                if a == idx || b == idx {
                    if let Some(w) = self.graph.edge_weight(edge) {
                        total += w.capacity_beds;
                    }
                }
            }
        }
        total
    }

    fn find_path(&self, start: NodeIndex, goal: NodeIndex) -> Option<Vec<NodeIndex>> {
        self.find_path_avoiding(start, goal, &HashSet::new())
    }

    fn find_path_avoiding(
        &self,
        start: NodeIndex,
        goal: NodeIndex,
        avoid: &HashSet<NodeIndex>,
    ) -> Option<Vec<NodeIndex>> {
        if avoid.contains(&start) || avoid.contains(&goal) {
            return None;
        }
        let mut visited: HashSet<NodeIndex> = HashSet::new();
        let mut pred: HashMap<NodeIndex, NodeIndex> = HashMap::new();
        let mut queue: VecDeque<NodeIndex> = VecDeque::new();
        visited.insert(start);
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            if node == goal {
                let mut path = vec![goal];
                let mut cur = goal;
                while cur != start {
                    match pred.get(&cur) {
                        Some(&p) => {
                            path.push(p);
                            cur = p;
                        }
                        None => return None,
                    }
                }
                path.reverse();
                return Some(path);
            }
            for neighbor in self.graph.neighbors(node) {
                if avoid.contains(&neighbor) || visited.contains(&neighbor) {
                    continue;
                }
                visited.insert(neighbor);
                pred.insert(neighbor, node);
                queue.push_back(neighbor);
            }
        }
        None
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facility(id: &str, role: FacilityRole) -> Facility {
        Facility {
            id: id.to_string(),
            name: format!("name-{id}"),
            role,
        }
    }

    fn referral(id: &str, capacity_beds: f64, basis: DemandBasis) -> Referral {
        Referral {
            id: id.to_string(),
            capacity_beds,
            basis,
        }
    }

    #[test]
    fn build_counts_and_degree() {
        let mut net = Network::new();
        net.add_facility(facility("a", FacilityRole::Hospital))
            .unwrap();
        net.add_facility(facility("b", FacilityRole::Clinic))
            .unwrap();
        net.add_facility(facility("c", FacilityRole::Specialty))
            .unwrap();
        net.add_referral("a", "b", referral("r1", 10.0, DemandBasis::Baseline))
            .unwrap();
        net.add_referral("b", "c", referral("r2", 5.0, DemandBasis::Surge))
            .unwrap();

        assert_eq!(net.facility_count(), 3);
        assert_eq!(net.referral_count(), 2);
        assert_eq!(net.degree("b"), Some(2));
        assert_eq!(net.degree("a"), Some(1));
        assert_eq!(net.degree("missing"), None);
    }

    #[test]
    fn connectivity_across_gap() {
        let mut net = Network::new();
        net.add_facility(facility("a", FacilityRole::Hospital))
            .unwrap();
        net.add_facility(facility("b", FacilityRole::Clinic))
            .unwrap();
        net.add_facility(facility("x", FacilityRole::Specialty))
            .unwrap();
        net.add_referral("a", "b", referral("r1", 3.0, DemandBasis::Baseline))
            .unwrap();

        assert!(net.is_connected("a", "b"));
        assert!(!net.is_connected("a", "x"));
    }

    #[test]
    fn incident_capacity_sums() {
        let mut net = Network::new();
        net.add_facility(facility("a", FacilityRole::Hospital))
            .unwrap();
        net.add_facility(facility("b", FacilityRole::Clinic))
            .unwrap();
        net.add_facility(facility("c", FacilityRole::Specialty))
            .unwrap();
        net.add_referral("a", "b", referral("r1", 4.0, DemandBasis::Baseline))
            .unwrap();
        net.add_referral("a", "c", referral("r2", 6.5, DemandBasis::Surge))
            .unwrap();

        assert_eq!(net.incident_capacity_beds("a"), 10.5);
        assert_eq!(net.incident_capacity_beds("b"), 4.0);
        assert_eq!(net.incident_capacity_beds("missing"), 0.0);
    }

    #[test]
    fn referral_basis_preserved() {
        let mut net = Network::new();
        net.add_facility(facility("a", FacilityRole::Hospital))
            .unwrap();
        net.add_facility(facility("b", FacilityRole::Clinic))
            .unwrap();
        net.add_facility(facility("c", FacilityRole::Specialty))
            .unwrap();
        net.add_referral("a", "b", referral("surge", 2.0, DemandBasis::Surge))
            .unwrap();
        net.add_referral("b", "c", referral("base", 2.0, DemandBasis::Baseline))
            .unwrap();

        let mut surge = None;
        let mut baseline = None;
        for e in net.graph.edge_indices() {
            let w = net.graph.edge_weight(e).unwrap();
            if w.id == "surge" {
                surge = Some(w.basis);
            } else if w.id == "base" {
                baseline = Some(w.basis);
            }
        }
        assert_eq!(surge, Some(DemandBasis::Surge));
        assert_eq!(baseline, Some(DemandBasis::Baseline));
    }

    #[test]
    fn diverse_path_ring_versus_chain() {
        let mut ring = Network::new();
        for id in ["a", "b", "c", "d"] {
            ring.add_facility(facility(id, FacilityRole::Hospital))
                .unwrap();
        }
        ring.add_referral("a", "b", referral("e1", 1.0, DemandBasis::Baseline))
            .unwrap();
        ring.add_referral("b", "c", referral("e2", 1.0, DemandBasis::Baseline))
            .unwrap();
        ring.add_referral("c", "d", referral("e3", 1.0, DemandBasis::Baseline))
            .unwrap();
        ring.add_referral("d", "a", referral("e4", 1.0, DemandBasis::Baseline))
            .unwrap();
        assert!(ring.has_diverse_path("a", "c"));

        let mut chain = Network::new();
        for id in ["a", "b", "c"] {
            chain
                .add_facility(facility(id, FacilityRole::Hospital))
                .unwrap();
        }
        chain
            .add_referral("a", "b", referral("e1", 1.0, DemandBasis::Baseline))
            .unwrap();
        chain
            .add_referral("b", "c", referral("e2", 1.0, DemandBasis::Baseline))
            .unwrap();
        assert!(!chain.has_diverse_path("a", "c"));
    }

    #[test]
    fn rejects_duplicate_facility() {
        let mut net = Network::new();
        net.add_facility(facility("a", FacilityRole::Hospital))
            .unwrap();
        let err = net
            .add_facility(facility("a", FacilityRole::Clinic))
            .unwrap_err();
        assert!(matches!(err, NetworkError::DuplicateFacility(id) if id == "a"));
    }

    #[test]
    fn rejects_bad_referrals() {
        let mut net = Network::new();
        net.add_facility(facility("a", FacilityRole::Hospital))
            .unwrap();
        net.add_facility(facility("b", FacilityRole::Clinic))
            .unwrap();

        let cap_err = net
            .add_referral("a", "b", referral("r1", 0.0, DemandBasis::Baseline))
            .unwrap_err();
        assert!(matches!(cap_err, NetworkError::NonPositiveCapacity(c) if c == 0.0));

        let unknown_err = net
            .add_referral("a", "z", referral("r2", 1.0, DemandBasis::Baseline))
            .unwrap_err();
        assert!(matches!(unknown_err, NetworkError::UnknownFacility(id) if id == "z"));
    }
}
