use crate::prelude::*;

pub struct Day11;
impl Day for Day11 {
    fn star1(&self, input: String) -> String {
        input
            .parse::<Galaxy>()
            .unwrap()
            .expand(2)
            .stars()
            .tuple_combinations()
            .map(|(i1, i2)| dist(i1, i2))
            .sum::<usize>()
            .to_string()
    }
    fn star2(&self, input: String) -> String {
        let e2 = input
            .parse::<Galaxy>()
            .unwrap()
            .expand(2)
            .stars()
            .tuple_combinations()
            .map(|(i1, i2)| dist(i1, i2))
            .sum::<usize>();

        let e3 = input
            .parse::<Galaxy>()
            .unwrap()
            .expand(3)
            .stars()
            .tuple_combinations()
            .map(|(i1, i2)| dist(i1, i2))
            .sum::<usize>();

        ((1_000_000 - 2) * (e3 - e2) + e2).to_string()
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
            .map(|mut col| col.all(|c| *c == '.').then_some('+'))
            .collect_vec();

        // expand cols
        (&mut self.data).into_iter().for_each(|row| {
            *row = row
                .iter_mut()
                .map(|c| Some(*c))
                .interleave(cols_to_expand.iter().cloned())
                .flatten()
                .flat_map(|c| {
                    (c == '+')
                        .then(|| vec!['+'; size - 1])
                        .unwrap_or_else(|| vec![c])
                })
                .collect()
        });

        //expand rows
        self.data = self
            .data
            .into_iter()
            .flat_map(|row| {
                if row.iter().all(|c| *c != '#') {
                    vec![row.into_iter().map(|_| '.').collect_vec(); size]
                } else {
                    vec![row]
                }
            })
            .collect();

        self
    }

    fn stars(&self) -> impl '_ + Iterator<Item = Index> + Clone {
        self.data.rows().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .flat_map(move |(j, c)| (*c == '#').then_some((i, j)))
        })
    }

    #[allow(unused)]
    fn display(&self) -> &Self {
        println!("{}", self.data);
        self
    }
}

fn dist(i1: Index, i2: Index) -> usize {
    i1.0.abs_diff(i2.0) + i1.1.abs_diff(i2.1)
}
