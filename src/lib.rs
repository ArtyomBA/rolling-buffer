pub mod buffer;

#[cfg(test)]
mod tests {
    use crate::buffer::{buffer::RollingBuffer, traits::Rolling};
    
    #[test]
    fn test_rolling_data_underflow() {
        let mut data = RollingBuffer::<i32>::new(4);
        data.push(1);
        data.push(2);

        assert_eq!(*data.raw(), [1, 2, 0, 0]);
        assert_eq!(*data.to_vec(), [1, 2]);
        assert_eq!(*data.last().unwrap_or(&0), 2);
        assert_eq!(*data.first().unwrap_or(&0), 1);
        assert_eq!(data.size(), 4);
        assert_eq!(data.count(), 2);
        assert_eq!(data.last_removed().unwrap(), 0);
    }
    
    #[test]
    fn test_rolling_data_overflow() {
        let mut data = RollingBuffer::<i32>::new(4);
        data.push(1);
        data.push(2);
        data.push(3);
        data.push(4);
        data.push(5);
        data.push(6);

        assert_eq!(*data.raw(), [5, 6, 3, 4]);
        assert_eq!(*data.to_vec(), [3, 4, 5, 6]);
        assert_eq!(*data.last().unwrap_or(&0), 6);
        assert_eq!(*data.first().unwrap_or(&0), 3);
        assert_eq!(data.size(), 4);
        assert_eq!(data.count(), 6);
        assert_eq!(data.last_removed().unwrap(), 2);
        assert_eq!(*data.get(3).unwrap(), 4);
        assert_eq!(*data.get(4).unwrap(), 5);
        assert_eq!(*data.get(5).unwrap(), 6);
    }

    #[test]
    fn test_as_vec() {
        let mut data = RollingBuffer::<i32>::new(4);
        data.push(1);
        data.push(2);
        data.push(3);
        assert_eq!(data.to_vec(), [1, 2, 3]);
        data.push(4);
        data.push(5);
        assert_eq!(data.count(), 5);
        assert_eq!(data.size(), 4);
        assert_eq!(data.to_vec(), [2, 3, 4, 5]);
    }

    #[test]
    fn test_size_0() {
        let mut data = RollingBuffer::<i32>::new(0);
        data.push(1);
        data.push(2);
        data.push(3);
        assert_eq!(data.to_vec(), [1, 2, 3]);
        data.push(4);
        data.push(5);
        assert_eq!(data.count(), 5);
        assert_eq!(data.size(), 0);
        assert_eq!(data.to_vec(), [1, 2, 3, 4, 5]);
    }
}