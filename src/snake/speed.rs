use strum::IntoEnumIterator;

// Modèle de vitesse prédéfinie avec des valeurs entières (u64)
#[derive(Debug, PartialEq, strum::Display, strum::EnumIter, Clone)]
pub enum Velocity {
    Slow = 75,
    Normal = 100,
    Fast = 150,
    Tremendous = 200,
}
//Reflexion to have a constant speed instead !
pub struct Speed {
    current: u64,
    step: u64, // enhance speed
    min: u64,
    max: u64,
}

impl From<Velocity> for u64 {
    fn from(speed: Velocity) -> Self {
        speed as u64
    }
}

impl Speed {
    pub fn new(initial_speed: Velocity, step: u64) -> Speed {
        Speed {
            current: u64::from(initial_speed),
            step,
            min: u64::from(Velocity::Slow),
            max: u64::from(Velocity::Tremendous),
        }
    }
    pub fn speed_up(&mut self) {
        if self.current + self.step > self.max {
            self.current = self.max;
        } else {
            self.current += self.step;
        }
    }

    pub fn speed_down(&mut self) {
        if self.current - self.step < self.min {
            self.current = self.min;
        } else {
            self.current -= self.step;
        }
    }

    pub fn get_speed(&self) -> u64 {
        self.current
    }

    pub fn get_speed_level_string(&self) -> String {
        for predefined_speed in Velocity::iter() {
            let speed_limit: u64 = u64::from(predefined_speed.clone());
            if self.current < speed_limit {
                return predefined_speed.to_string();
            }
        }
        Velocity::Tremendous.to_string()
    }
}
