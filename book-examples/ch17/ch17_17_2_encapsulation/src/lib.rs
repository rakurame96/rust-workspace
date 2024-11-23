#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();

        self.average = total as f64 / self.list.len() as f64; 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = AveragedCollection {
            list: vec![1, 2, 3],
            average: 0.0,
        };
        result.add(4);
        assert_eq!(result.average, 2.5, "The average should be 2.5.");
        assert_eq!(result.list, vec![1, 2, 3, 4], "The list should match the input vector.");
    }
}
