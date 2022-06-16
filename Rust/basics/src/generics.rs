// example max only on i32
fn max_i32(array: &[i32]) -> Result<i32, bool> {
    return if array.len() == 0 {
        println!("the given array is empty!");
        Err(false)
    } else {
        let mut max_value = array[0];
        for value in &array[1..] {
            max_value = if max_value > *value { max_value } else { *value };
        }
        Ok(max_value)
    }
}

pub fn test_max_i32(array: &[i32]) -> i32 {
    let result = max_i32(array);
    return match result {
        Ok(v) => v,
        Err(e) => -1
    }
}

/** this function doesn't compile because undefined behavior > in type T
 *  fn max<T>(array: &[T]) -> T {
 *      let mut max_index = 0;
 *      let mut i = 1;
 *      while i < array.len() {
 *          if array[i] > array[max_index] {
 *              max_index = i;
 *          }
 *          i += 1;
 *      }
 *      array[max_index]
 *  }
 */

pub struct Point<T> {
    x: T,
    y: T,
}

// only have Clone trait can implement these method
impl<T: Clone> Point<T> {
    pub fn get_x(&self) -> &T {
        &self.x
    }

    pub fn get_y(&self) -> &T {
        &self.y
    }

    pub fn create(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

// only implement for one specific type
impl Point<f64> {
    pub fn get_distance(&self, point: &Point<f64>) -> f64 {
        f64::sqrt((&self.x - &point.x).powi(2) + (&self.y - &point.y).powi(2))
    }
}

// the generic type is not bind to some specific letters
pub struct Texel<S, T> {
    pub s: S,
    pub t: T,
}

impl<S, T> Texel<S, T> {
    pub fn mix<U, V>(self, other: Texel<U, V>) -> Texel<S, V> {
        Texel { s: self.s, t: other.t }
    }
}
