use std::convert::TryFrom;
use rand::Rng;

/// Permutation struct. Contains a permutation vector.
pub struct Permutation<T> {
    pub permu : Vec<T>,
}

impl<T> Permutation<T> where 
    T : Copy +
    TryFrom<u8> +
    TryFrom<usize> +
    PartialEq<T> +
    rand::distributions::range::SampleRange +
    std::cmp::PartialOrd +
    std::ops::Sub +
    std::fmt::Display + // NOTE : For debugging
    std::fmt::Debug // NOTE : For debugging
{
    /// Initializes a Permutation with the given vector. If the 
    /// given vector is not a permutation the function will return an Error. 
    ///
    /// # Example
    /// ```
    /// use permu_rs::Permutation;
    /// let vec : Vec<u16> = vec![0,1,2,3,4];
    /// let permu = Permutation::from_vec(vec);
    /// ```
    pub fn from_vec(vec: Vec<T>) -> Result<Permutation<T>, & 'static str> {
        /*
        let identity = Self::identity(vec.len())?;
        let is_permu = identity.permu.iter() 
            .all(|i| vec.iter().fold(0, |acc, x| usize::from(*x == *i)) == 1);
        */
        let permu = Permutation {permu : vec};
        
        match permu.is_permu() {
            true => Ok(permu),
            false => Err("The given vector is not a permutation"),
        }
    }

    /// Initializes a Permutation with the given vector.
    /// No checking is done to the given permutation, the
    /// permutation can be initialized with a vector that 
    /// is not a real permutation. It is faster than `from_vec`.
    /// 
    /// # Example
    /// ```
    /// use permu_rs::Permutation;
    /// let vec : Vec<u16> = vec![0,1,2,3,4];
    /// let permu : Permutation<u16> = Permutation::from_vec_unsec(vec);
    /// ```
    pub fn from_vec_unsec(vec: Vec<T>) -> Permutation<T> {
        Permutation { permu : vec }
    }

    /// Generates a random permutation of the length given.
    ///
    /// # Panics
    /// If the length given is grater than the maximum value that `T` can hold,
    /// the method will panic.
    ///
    /// # Example
    /// ```
    /// use permu_rs::Permutation;
    /// let rand_permu : Permutation<u16> = Permutation::random(8);
    /// assert!(rand_permu.is_permu());
    /// assert_eq!(8, rand_permu.permu.len());
    /// ```
    pub fn random(length: usize) -> Permutation<T> {
        let mut permu: Vec<T> = Vec::with_capacity(length);

        let zero = match Self::zero() {
            Ok(z) => z,
            Err(e) => panic!(e),
        };

        let max = match T::try_from(length) {
            Ok(v) => v,
            Err(_) => panic!("Can not create a permutation longer than the max size of the its type"),
        };

        while permu.len() < length {  
            // Generate random number. n : [0, length)
            let n = rand::thread_rng().gen_range(zero, max);

            if !Self::find_unsec_asref(&permu, n) {
                permu.push(n);
            }
        }
        Permutation{ permu : permu }
    }

    // Returns zero value of type T
    fn zero() -> Result<T, & 'static str> {
        match T::try_from(0usize) {
            Ok(v) => Ok(v),
            Err(_) => Err("Zero conversion error"),
        }
    }
    
    /// Returns an identity vector of the length given.
    ///
    /// # Example
    /// ```
    /// use permu_rs::Permutation;
    /// let identity : Permutation<u8> = Permutation::identity(5).unwrap();
    /// assert_eq!(vec![0,1,2,3,4], identity.permu);
    /// ```
    pub fn identity(length: usize) -> Result<Permutation<T>, & 'static str> {
        let mut identity: Vec<T> = vec![Self::zero().unwrap();length];

        for i in 0..length  {
            identity[i] = match T::try_from(i) {
                Ok(v) => v,
                Err(_) => return Err("Conversion error"),
            }; 
        }
        Ok(Permutation { permu : identity })
    }
    
    /// Finds an element inside the permutation. 
    /// The type of the value to find is `usize`, so if its value is
    /// greater than `T::MAX_VALUE` or lower than `T::MIN_VALUE` the method
    /// return an an error. If boundaries were satisfied, the method 
    /// returns true if the element is found, if not, returns false.
    /// 
    /// # Example
    /// ```
    /// use permu_rs::Permutation;
    /// let permu : Permutation<u8> = Permutation::from_vec(vec![0,1,2,3]).unwrap();
    /// assert_eq!(Ok(true), permu.find(2));
    /// assert_eq!(Ok(false),permu.find(7));
    /// ```
    pub fn find(&self, item: usize) -> Result<bool, &'static str> {
        let v = match T::try_from(item) {
            Ok(v) => v,
            Err(_) => return Err("Conversion error"),
        };
        Ok(self.permu.iter().any(|&x| x == v))
    }

    /// Finds an element inside the permutation. 
    ///
    /// # Panics
    /// The type of the value to find is `usize`, so if its value is
    /// greater than `T::MAX_VALUE` or lower than `T::MIN_VALUE` the method
    /// will panic.
    /// 
    /// # Example
    /// ```
    /// use permu_rs::Permutation;
    /// let permu : Permutation<u8> = Permutation::from_vec(vec![0,1,2,3]).unwrap();
    /// assert_eq!(Ok(true), permu.find(2));
    /// assert_eq!(Ok(false),permu.find(7));
    /// ```
    pub fn find_unsec(&self, item: usize) -> bool {
        let v = match T::try_from(item) {
            Ok(v) => v,
            Err(_) => panic!("Conversion error"),
        };
        self.permu.iter().any(|&x| x == v)
    }
    
    // Private implementation of find_unsec function.
    // Takes a permutation and an item to find.
    // WARNING: As find_unsec method, this function can panic.
    fn find_unsec_asref(permu: &Vec<T>, item: T) -> bool {
        let v = match T::try_from(item) {
            Ok(v) => v,
            Err(_) => panic!("Conversion error"),
        };
        permu.iter().any(|&x| x == v)
    }
    
    /// Checks if the vector inside `Permutation` is really a permutation.
    ///
    /// # Panics
    /// This method internally uses `find_unsec` method, and this method can panic.
    /// However, this method will never panic as it always request to find values that are 
    /// inside the range T::MIN_VALUE and T::MAX_VALUE.
    ///
    /// # Example
    /// ```
    /// use permu_rs::Permutation;
    /// let permu1 : Permutation<u8> = Permutation::from_vec_unsec(vec![0,1,2,3]);
    /// let permu2 : Permutation<u8> = Permutation::from_vec_unsec(vec![1,2,3]);
    /// let permu3 : Permutation<u8> = Permutation::from_vec_unsec(vec![0,1,4,3]);
    /// let permu4 : Permutation<u8> = Permutation::from_vec_unsec(vec![0,1,1,3]);
    ///
    /// assert!(permu1.is_permu());
    /// assert!(!permu2.is_permu()); // Not permutation
    /// assert!(!permu3.is_permu()); // Not permutation
    /// assert!(!permu4.is_permu()); // Not permutation
    /// ```
    pub fn is_permu(&self) -> bool {
        (0..self.permu.len()).all(|i| self.find_unsec(i))
    }
}

#[cfg(test)]
mod tests_permu {

    use crate::Permutation;

    #[test]
    fn find_unsec_asref() {
        let v = vec![1,2,3];
        v.iter().for_each(|x| assert!(
                Permutation::find_unsec_asref(&v, *x)));
    }

    #[test]
    fn find_unsec_asref2() {
        let mut v : Vec<u32> = Vec::with_capacity(5);
        assert!(!Permutation::find_unsec_asref(&v, 3));
        
        v.push(3);
        assert!(Permutation::find_unsec_asref(&v, 3));
    }

    #[test]
    fn generate_rand_permus() {
        for _i in 0..1000 {
            let permu : Permutation<u8> = Permutation::random(40);
            assert!(permu.is_permu());
        }
    }
}
