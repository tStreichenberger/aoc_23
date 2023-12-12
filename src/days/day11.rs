use crate::prelude::*;

pub struct Day11;
impl Day for Day11 {
    fn star1(&self, input: String) -> String {
        input
            .parse::<Galaxy>()
            .unwrap()
            .expand(1)
            .display()
            .stars()
            .tuple_combinations()
            .map(|(i1, i2)| dist(i1, i2))
            .sum::<usize>()
            .to_string()
    }
}

struct Galaxy {
    data: Grid<char>,
}

impl FromStr for Galaxy {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            data: s.parse().unwrap(),
        })
    }
}

impl Galaxy {
    fn expand(mut self, size: usize) -> Self {
        // find cols to expand
        let cols_to_expand = self
            .data
            .cols()
            .map(|mut col| col.all(|c| *c == '.').then_some('_'))
            .collect_vec();

        // expand cols
        (&mut self.data).into_iter().for_each(|row| {
            *row = row
                .into_iter()
                .map(|c| Some(*c))
                .interleave(cols_to_expand.iter().cloned())
                .flatten()
                .flat_map(|c| {
                    (c == '_')
                        .then(|| vec!['_'; size])
                        .unwrap_or_else(|| vec![c])
                })
                .collect()
        });

        //expand rows
        self.data = self
            .data
            .into_iter()
            .flat_map(|row| {
                if row.iter().all(|c| *c == '.') {
                    vec![row; size]
                } else {
                    vec![row]
                }
            })
            .collect();

        self
    }

    fn stars<'a>(&'a self) -> impl 'a + Iterator<Item = Index> + Clone {
        self.data
            .rows()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(j, c)| (*c == '#').then_some((i, j)))
                    .flatten()
            })
            .flatten()
    }

    fn display(&self) -> &Self {
        println!("{}", self.data);
        self
    }
}

fn dist(i1: Index, i2: Index) -> usize {
    i1.0.abs_diff(i2.0) + i1.1.abs_diff(i2.1)
}
