/// DisjointSet type: represents a universe of n elements distributed
/// among a family of disjoint sets. Each element is an index from 0
/// to n - 1. Each disjoint set is represented by one of its elements
pub struct DisjointSet {
    // each element has a parent. When an element is parent of itself,
    // it is called a representative
    parent: Vec<usize>,
    // each element has a rank, that is, an upper bound to its height
    rank: Vec<usize>,
}
// alias used to represent the return value of functions that can
// return an error
type Result<'a> = std::result::Result<(), &'a str>;

// constructor
impl DisjointSet {
    /// creates a DisjointSet with num_elements elements, each one
    /// forming a singleton disjoint set
    pub fn new(num_elements: usize) -> Self {
        let mut parent = Vec::new();
        let mut rank = Vec::new();
        // each elements starts as a representative ...
        for e in 0..num_elements {
            parent.push(e);
        }
        // ... and with zero rank
        rank.resize(num_elements, 0);

        Self { parent, rank }
    }
}

// auxiliar functions
impl DisjointSet {
    // determines whether element is valid for this DisjointSet
    fn valid(&self, element: usize) -> bool {
        element < self.parent.len()
    }
    // turns rep_a representative into an regular element represented
    // by rep_b representative
    fn make_represented_by(&mut self, rep_a: usize, rep_b: usize) {
        self.parent[rep_a] = rep_b;
    }
}

// accessors
impl DisjointSet {
    /// returns representative of element, in case element is valid
    /// for this DisjointSet. This method gets a mutable reference in
    /// order to implement path compression
    pub fn representative(&mut self, element: usize) -> Option<usize> {
        // if element is valid
        if self.valid(element) {
            // if it is a representative, return it
            if self.parent[element] == element {
                Some(element)
            } else {
                // otherwise, get its parent representative, also its
                // own representative
                let result = self.representative(self.parent[element]);
                // path compression: assign its representative
                // directly as its parent
                self.parent[element] = result.unwrap();
                // returns representative
                result
            }
        } else {
            // element is invalid: return None
            None
        }
    }
}

// modifiers
impl DisjointSet {
    /// turns disjoint sets containing element_a and element_b into an
    /// unique disjoint set
    pub fn join(&mut self, element_a: usize, element_b: usize) -> Result {
        // if both elements are valid
        if self.valid(element_a) && self.valid(element_b) {
            // get their representatives
            let rep_a = self.representative(element_a).unwrap();
            let rep_b = self.representative(element_b).unwrap();

            // lower rank representative becomes represented by higher
            // rank representative
            if self.rank[rep_a] < self.rank[rep_b] {
                self.make_represented_by(rep_a, rep_b);
            } else if self.rank[rep_a] > self.rank[rep_b] {
                self.make_represented_by(rep_b, rep_a);
            } else {
                // in case of equal rank, rep_b continues to be a
                // representative ...
                self.make_represented_by(rep_a, rep_b);
                // ... but its rank increases
                self.rank[rep_b] += 1;
            }

            Ok(())
        } else {
            // in case at least one element is invalid, signals an error
            Err("DisjointSet: attempt to join invalid elements")
        }
    }
}
