
pub struct Queue<T, const N: usize> {
    data: [Option<T>; N],
    front: usize,
    back: usize,
    size: usize,
}

impl<T, const N: usize> Queue<T, N> {
    pub fn new() -> Self {
        Queue {
            data: [(); N].map(|_| None),
            front: 0,
            back: 0,
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.size >= N {
            return Err(item); // Queue is full
        }

        self.data[self.back] = Some(item);
        self.back = (self.back + 1) % N;
        self.size += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None; // Queue is empty
        }

        let item = self.data[self.front].take();
        self.front = (self.front + 1) % N;
        self.size -= 1;
        item
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None; // Index out of bounds
        }

        let idx = (self.front + index) % N;
        self.data[idx].as_ref()
    }
}

pub fn exp(x: f64) -> f64 {
    const MAX_ITER: usize = 10; // Maximum number of iterations for the Taylor series
    let mut sum = 1.0; // Start with the first term of the series
    let mut term = 1.0; // This will hold x^n / n!

    for i in 1..=MAX_ITER {
        term *= x / (i as f64); // Calculate the next term in the series
        sum += term; // Add the term to the sum
    }

    sum
}


#[allow(dead_code)]
pub fn abs(a: i32) -> i32{
    if a < 0 {
        -a
    }
    else {
        a
    }
}

pub fn sqrt(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }

    let mut guess = x;
    const EPSILON: f64 = 1e-5;
    let mut prev_guess: f64;

    loop {
        prev_guess = guess;
        guess = (guess + x / guess) / 2.0;

        // Check for convergence
        let delta = prev_guess - guess;
        if delta >= 0. {
            if delta < EPSILON {
                break;
            }
        }
        else {
            if delta > -EPSILON {
                break;
            }
        }
    }

    if guess >= 0. {
        guess
    }
    else {
        -guess
    }
}

// pub fn f64_to_u32(value: f64) -> u32 {
//     if value.is_nan() {
//         return u32::MIN;
//     }
//     if value.is_infinite() {
//         return u32::MAX;
//     }
//     if value < u32::MIN as f64 {
//         return u32::MIN;
//     }
//     if value > u32::MAX as f64 {
//         return u32::MAX;
//     }
//     value as u32
// }