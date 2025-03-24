use std::cmp::min;

use super::traits::Rolling;
/// RollingBuffer is a fixed size heap buffer that will override the beginning of the buffer when it is full
/// RollingBuffer is a very simple Vec wrapper that only uses safe code.
/// 
/// ['size']: size is the maximum number of elements that the buffer can hold
/// ['vec']: vec is the underlying Vec that stores the elements of the buffer
/// ['last_removed']: last_removed is the last element that was removed from the buffer
/// ['count']: count is the number of elements in the buffer as if the buffer was Vec
#[derive(Debug, Clone, Default)]
pub struct RollingBuffer<T>
where
    T: Clone
{
    size: usize,
    vec: Vec<T>,
    last_removed: Option<T>,
    count: usize,
}


impl<T> Rolling<T> for RollingBuffer<T> 
where
    T: Clone + Default
{
    /// Creates a new RollingBuffer with the given size and initial value (aka none)
    /// If the size is 0, the buffer will behave as a normal Vec
    fn new(size: usize) -> Self {
        Self {
            size,
            vec: if size > 0 {
                vec![T::default(); size]
            } else {
                Vec::new()
            },
            last_removed: None,
            count: 0,
        }
    }

    
    /// Adds an element to the buffer, overriding the beginning of the buffer when it is full
    /// Here using "safe code", but it is essentially unsafe ptr::write()
    fn push(&mut self, value: T) {
        if self.size > 0 {
            let index = self.count as usize % self.size;
            self.last_removed = Some(std::mem::replace(&mut self.vec[index], value));
        } else {
            self.vec.push(value);
        }
        self.count += 1;
    }

    
    /// Get the element at the given index, as if the buffer was a Vec
    /// 
    /// buffer of size 3, adding 1,2,3,4 and asking for the element at index 3 will return 4.
    /// Asking for index 0 will return None
    /// since this element was overriden already.
    /// Example:
    /// ```
    /// let mut buffer = RollingBuffer::<i32>::new(3, 0);
    /// buffer.push(1);
    /// buffer.push(2);
    /// buffer.push(3);
    /// buffer.push(4);
    /// assert_eq!(buffer.get(3), Some(&4));
    /// assert_eq!(buffer.get(0), None);
    /// ```
    fn get(&self, i: usize) -> Option<&T> {
        if self.size > 0 {
            Some(&self.vec[i % self.size])
        } else if i < self.vec.len() {
            Some(&self.vec[i])
        } else {
            None
        }
    }

    /// Returns an option containing a reference to the first element in the rolling data.
    ///
    /// If no elements have been added (`count` is zero), it returns `None`.
    /// Otherwise, it returns a reference to the last added element.
    /// The index calculation considers the possibility of wrapping around when
    /// the number of elements added exceeds the size of the vec.
    fn last(&self) -> Option<&T> {
        if self.count == 0 {
            None
        } else if self.size > 0 {
            let index = (self.count as usize - 1) % self.size;
            Some(&self.vec[index])
        } else {
            Some(&self.vec[self.vec.len() - 1])
        }
    }

    /// Last added element's mutable reference.
    fn last_mut(&mut self) -> Option<&mut T> {
        if self.count == 0 {
            None
        } else if self.size > 0 {
            let index = (self.count as usize - 1) % self.size;
            Some(&mut self.vec[index])
        } else {
            let index = self.vec.len() - 1;
            Some(&mut self.vec[index])
        }
    }

    /// Returns the theoretical first element.
    /// 
    /// Example: 
    /// ```
    /// let mut buffer = RollingBuffer::<i32>::new(3);
    /// buffer.push(1);
    /// buffer.push(2);
    /// buffer.push(3);
    /// buffer.push(4);
    /// assert_eq!(buffer.first(), Some(&2));
    /// ```
    fn first(&self) -> Option<&T> {
        if self.count == 0 {
            None
        } else if self.size > 0 {
            if self.count <= self.size {
                Some(&self.vec[0])
            } else {
                let index = (self.count as usize) % self.size;
                Some(&self.vec[index])
            }
        } else {
            Some(&self.vec[0])
        }
    }

    /// Returns theoretical len as if it was a Vec.
    fn len(&self) -> usize {
        if self.count < self.size {
            self.count as usize
        } else {
            self.vec.len()
        }
    }

    /// Returns the maximum number of elements that can be stored.
    fn size(&self) -> usize {
        self.size
    }

    /// Returns the underlying vector as it is stored inside the RollingBuffer.
    fn raw(&self) -> &Vec<T> {
        &self.vec
    }

    /// Returns the last removed element. Can be very useful if needed for debugging or other purposes.
    fn last_removed(&self) -> &Option<T> {
        &self.last_removed
    }
 
    /// Returns 'expected' number of elements as if the RollingBuffer was a Vec.
    /// i.e. the number of elements that would be in the Vec if it was not a RollingBuffer.
    fn count(&self) -> usize {
        self.count as usize
    }

    /// Returns true if the RollingBuffer is empty.
    fn is_empty(&self) -> bool {
        self.count == 0
    }
    
    /// Creates a new Vec, which contains all elements in the RollingBuffer in correct order.
    fn to_vec(&self) -> Vec<T> {
        if self.size > 0 {
            let start = if self.count <= self.size {
                0 as usize
            } else {
                self.count % self.size
            };
            let mut vec = Vec::<T>::new();
            for i in start..start + min(self.size, self.count) {
                vec.push(self.vec[i % self.size].clone());
            }
            vec
        } else {
            self.vec.clone()
        }
    }
}
