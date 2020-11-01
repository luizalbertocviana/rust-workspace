pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

type Result<'a> = std::result::Result<(), &'a str>;

// constructor
impl DisjointSet {
    pub fn new(num_elements: usize) -> Self {
        let mut parent = Vec::new();
        let mut rank = Vec::new();

        for e in 0..num_elements {
            parent.push(e);
        }

        rank.resize(num_elements, 0);

        Self { parent, rank }
    }
}

// auxiliar functions
impl DisjointSet {
    fn valid(&self, element: usize) -> bool {
        element < self.parent.len()
    }

    fn make_represented_by(&mut self, rep_a: usize, rep_b: usize) {
        self.parent[rep_a] = rep_b;
    }
}

// accessors
impl DisjointSet {
    pub fn representative(&mut self, element: usize) -> Option<usize> {
        if self.valid(element) {
            if self.parent[element] == element {
                Some(element)
            } else {
                let result = self.representative(self.parent[element]);
                self.parent[element] = result.unwrap();

                result
            }
        } else {
            None
        }
    }
}

// modifiers
impl DisjointSet {
    pub fn join(&mut self, element_a: usize, element_b: usize) -> Result {
        if self.valid(element_a) && self.valid(element_b) {
            let rep_a = self.representative(element_a).unwrap();
            let rep_b = self.representative(element_b).unwrap();

            if self.rank[rep_a] < self.rank[rep_b] {
                self.make_represented_by(rep_a, rep_b);
            } else if self.rank[rep_a] > self.rank[rep_b] {
                self.make_represented_by(rep_b, rep_a);
            } else {
                self.make_represented_by(rep_a, rep_b);

                self.rank[rep_b] += 1;
            }

            Ok(())
        } else {
            Err("DisjointSet: attempt to join invalid elements")
        }
    }
}
