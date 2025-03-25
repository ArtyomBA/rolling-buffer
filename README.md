# rolling-buffer
A simple Vec-like circular heap buffer.

The idea of this buffer is to wrap a normal Vec, so that at any time you can access it directly by calling '.raw()'.

The buffer is supposed to be used as a cache buffer, where you do not need to remove elements, but rather add them and only keep the last N elements.

This buffer can not ever grow and always has the fixed size.

This behaviour of the buffer is similar to VecDeque, but with limited functionality.

Example usage:
```
let mut buffer = RollingBuffer::<i32>::new(3);

buffer.push(1);
buffer.push(2);
buffer.push(3);
buffer.push(4);
buffer.push(5);

assert_eq!(*buffer.raw(), [4, 5, 3]);
assert_eq!(buffer.to_vec(), [3, 4, 5]);
assert_eq!(*buffer.get(1).unwrap(), 4);
assert_eq!(*buffer.first().unwrap(), 3);
assert_eq!(*buffer.last().unwrap(), 5);
```
