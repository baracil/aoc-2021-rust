use std::str::FromStr;

#[derive(Clone,Copy)]
pub struct School {
    fish_population:[u128;9],
}

impl School {

    pub fn compute_population(&self, t:&[u128;9]) -> u128 {
        (0..9).map(|i| self.fish_population[i] * t[i]).sum()
    }

    #[allow(dead_code)]
    pub fn single_fish(timer:usize) -> Self {
        let mut fish_population = [0;9];
        fish_population[timer] = 1;
        School{fish_population}
    }

    fn add_fish(&mut self,time:usize) -> &mut Self {
        self.fish_population[time] += 1;
        self
    }

    pub fn perform_one_tick(&mut self) {
        let n0 = self.fish_population[0];
        for i in 0..8 {
            self.fish_population[i] = self.fish_population[i+1];
        }
        self.fish_population[6]+=n0;
        self.fish_population[8]=n0;
    }

    pub fn population_size(&self) -> u128 {
        self.fish_population.iter().sum()
    }
}

impl Default for School {
    fn default() -> Self {
        School{ fish_population:[0;9]}
    }
}

impl FromStr for School {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {

        let mut school = School::default();

        let result = line.split(",")
            .map(|n| n.parse::<usize>().expect(&format!("Could not parse {}",n)))
            .fold(&mut school, |s, timer| s.add_fish(timer));

        Ok(School{fish_population:result.fish_population})
    }
}