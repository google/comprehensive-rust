# <code>Pin&lt;Ptr&gt;</code> and <code>Drop</code>

## Tension

- Pinning keeps values in place until they are dropped, but
- `Drop::drop()` receives `&mut self` rather than `self`

## Implementing `Drop` for `!Unpin` types

- Avoid the following:
  - assignment
  - `mem::take()`
  - `mem::swap()`

## Example

<!-- TODO: code example showing how to correctly drop an `!Unpin type-->
