use std::default::Default;

pub struct DigitSet {
    set: Vec<bool>,
}

impl DigitSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_size(num_digits: u32) -> Self {
        let len = 10_usize.pow(num_digits);
        Self {
            set: vec![false; len],
        }
    }

    pub fn reallocate(&mut self, num_digits: u32) {
        let current_len = self.set.len();
        let new_len = 10_usize.pow(num_digits);
        self.set.extend((current_len..new_len).map(|_| false))
    }

    pub fn contains(&self, num: &usize) -> bool {
        *self.set.get(*num).unwrap_or(&false)
    }

    // adds num to DigitSet. Returns true if the element was not already present
    pub fn insert(&mut self, num: usize) -> bool {
        self.set
            .get_mut(num)
            .map(|b| {
                let r = !*b;
                *b = true;
                r
            })
            .unwrap_or_else(|| {
                let new_size = f64::log10(num as f64).ceil() as u32;
                self.reallocate(new_size);
                self.insert(num)
            })
    }
}

impl FromIterator<usize> for DigitSet {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut set = DigitSet::new();
        iter.into_iter().for_each(|num| {
            set.insert(num);
        });
        set
    }
}

impl Default for DigitSet {
    fn default() -> Self {
        Self::new_with_size(2)
    }
}
