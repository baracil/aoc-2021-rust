use std::mem::discriminant;
use std::ops::Add;

#[derive(Clone)]
pub struct Distribution {
    left_element:u8,
    right_element:u8,
    distribution:[usize;26],
}

impl Distribution {
    pub fn amplitude(&self) -> usize {
        let (min,max) = self.distribution
            .iter()
            .filter(|u| **u>0)
            .fold((usize::MAX,usize::MIN), |(min,max),usize| (min.min(*usize), max.max(*usize)));

        max-min
    }
}

impl Distribution {

    pub fn first_generation(left:u8, right:u8) -> Distribution {
        let mut distribution = [0;26];

        distribution[left as usize] += 1;
        distribution[right as usize] += 1;

        Distribution{left_element:left,right_element:right,distribution}
    }
}

impl<'a,'b> Add<&'b Distribution> for &'a Distribution {
    type Output = Distribution;

    fn add(self, rhs: &'b Distribution) -> Self::Output {
        if self.right_element != rhs.left_element {
            panic!("Cannot add distribution with common element in the middle")
        }

        let mut distribution = [0;26];
        for (i, content) in distribution.iter_mut().enumerate() {
            *content = self.distribution[i] + rhs.distribution[i];
        }
        distribution[self.right_element as usize] -= 1 ;

        Distribution{left_element:self.left_element, right_element:rhs.right_element, distribution}

    }
}

impl Add for Distribution {
    type Output = Self;

    fn add(self, rhs: Distribution) -> Self::Output {
        if self.right_element != rhs.left_element {
            panic!("Cannot add distribution with common element in the middle")
        }

        let mut distribution = [0;26];
        for (i, content) in distribution.iter_mut().enumerate() {
            *content = self.distribution[i] + rhs.distribution[i];
        }
        distribution[self.right_element as usize] -= 1 ;

        Distribution{left_element:self.left_element, right_element:rhs.right_element, distribution}

    }
}


