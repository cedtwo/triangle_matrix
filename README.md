# Triangle Matrix

Upper and lower triangle matrix indexing operations.

Upper and lower triangle matrices indexing operations.
Provides the [`TriangleIndex`] trait for triangle matrix indexing operations
on a one dimensional collection. All operations are delegated to the inner
collection using `Deref` and `DerefMut`. Requires delegating the length of an
axis, `n`, using the [`TriangleType`] trait.

## Example

```rust
// Create a wrapper storing the length of an axis and the collection.
struct VecTri<Ty>(usize, Vec<usize>, PhantomData<Ty>);

// Implement `Deref` and `DerefMut`, delegating `Deref::Target` to the vector.
// ...

// Delegate `n` to the `usize` field and specify `Ty` as the triangle type.
impl<Ty> TriangleType<Ty> for VecTri<Ty> {
    fn n(&self) -> usize {
        self.0
    }
}

// A `4 * 4` upper triangle matrix where the elements are the usize indices.
let n = 4;
let v = Vec::from_iter(0..tri_num(n));
let mut m: VecTri<UpperTriangle> = VecTri(n, v, PhantomData);

assert_eq!(m.get_row_indices(0).collect::<Vec<_>>(), [0, 1, 2, 3]);
assert_eq!(m.get_row_indices(1).collect::<Vec<_>>(),    [4, 5, 6]);
assert_eq!(m.get_row_indices(2).collect::<Vec<_>>(),       [7, 8]);
assert_eq!(m.get_row_indices(3).collect::<Vec<_>>(),          [9]);

assert_eq!(m.get_element_index(1, 2), 6);
assert_eq!(*m.get_element(1, 2), 6);

assert_eq!(m.get_col_indices(3).collect::<Vec<_>>(), [3, 6, 8, 9]);

*m.get_element_mut(1, 2) = 10;
*m.get_element_mut(2, 1) = 11;

// Thanks to `Deref`, we can index into the underlying vector.
assert_eq!(m.get_col_indices(3).map(|i| m[i]).collect::<Vec<_>>(), [3, 10, 11, 9]);
````
Alternatively we can omit the generic type `Ty` on our struct, and qualify the
triangle type;

```rust
struct VecTri(usize, Vec<usize>);

// Implement `Deref` and `DerefMut`, delegating `Deref::Target` to the vector.
// ...

// Delegate `n` to the `usize` field and specify `Ty` as the triangle type.
impl<Ty> TriangleType<Ty> for VecTri {
    fn n(&self) -> usize {
        self.0
    }
}

let n = 4;
let v = Vec::from_iter(0..tri_num(n));
let m: VecTri = VecTri(n, v);

assert_eq!(
    <VecTri as TriangleIndex<UpperTriangle>>::get_col_indices(&m, 0).collect::<Vec<_>>(),
    [0]);
assert_eq!(<VecTri as TriangleIndex<LowerTriangle>>::get_col_indices(&m, 0).collect::<Vec<_>>(),
    [0, 1, 3, 6]);
```
## Indexing
A type implementing `TriangleIndex` (or `Deref` and `DerefMut`, for that matter)
would typically not be appropriate for a public facing API. Depending on what we
with to represent, we may need to manipulate indices prior to making calls to
`TriangleIndex`. Take for example a symmetric matrix using a **lower triangle**
data structure where we have no use of diagonal elements. This  could be
represented as the following;

```rust
/// A symmetric matrix with no diagonal elements.
struct SymmetricMatrix<T> {
    inner: T,
}

impl<T> SymmetricMatrix<T>
where
    T: TriangleType<LowerTriangle> + TriangleIndex<LowerTriangle>,
    T::Target: IndexMut<usize>,
    <T::Target as Index<usize>>::Output: Sized,
{
    /// The length of either axis of the array.
    fn n(&self) -> usize {
        self.inner.n() + 1
}

    /// All elements of the row `i`.
    fn get_row<'a>(
        &'a self,
        i: usize,
    ) -> Box<dyn Iterator<Item = &<T::Target as Index<usize>>::Output> + 'a> {
        if i == 0 {
            Box::new(
                self.inner
                    .get_col_indices(i)
                    .map(|index| &self.inner[index]),
            )
        } else {
            Box::new(
                self.inner
                    .get_row_indices(i - 1)
                    .chain(self.inner.get_col_indices(i))
                    .map(|index| &self.inner[index]),
            )
        }
    }
}

impl<T> Index<(usize, usize)> for SymmetricMatrix<T>
where
    T: TriangleType<LowerTriangle> + TriangleIndex<LowerTriangle>,
    T::Target: std::ops::IndexMut<usize>,
{
    type Output = <T::Target as Index<usize>>::Output;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        if i == j {
            panic!("Matrix type does not contain diagonal elements");
        } else if i < j {
            self.inner.get_element(j - 1, i)
        } else {
            self.inner.get_element(i - 1, j)
        }
    }
}

let n = 4;
let v = Vec::from_iter(0..tri_num(n));
let m = SymmetricMatrix {
    inner: VecTri::<_>(n, v, PhantomData),
};

/// A Symmetric matrix of 5 elements where we don't need the `i == j` diagonal.
assert_eq!(m.n(), 5);

assert_eq!(m.get_row(0).cloned().collect::<Vec<_>>(), [   0, 1, 3, 6]);
assert_eq!(m.get_row(1).cloned().collect::<Vec<_>>(), [0,    2, 4, 7]);
assert_eq!(m.get_row(2).cloned().collect::<Vec<_>>(), [1, 2,    5, 8]);
assert_eq!(m.get_row(3).cloned().collect::<Vec<_>>(), [3, 4, 5,    9]);
assert_eq!(m.get_row(4).cloned().collect::<Vec<_>>(), [6, 7, 8, 9,  ]);

assert_eq!(m[(0, 1)], 0);
assert_eq!(m[(0, 2)], 1);
assert_eq!(m[(0, 3)], 3);
assert_eq!(m[(0, 4)], 6);

assert_eq!(m[(1, 0)], 0);
assert_eq!(m[(1, 2)], 2);
assert_eq!(m[(1, 3)], 4);
assert_eq!(m[(1, 4)], 7);

assert_eq!(m[(2, 0)], 1);
assert_eq!(m[(2, 1)], 2);
assert_eq!(m[(2, 3)], 5);
assert_eq!(m[(2, 4)], 8);

assert_eq!(m[(3, 0)], 3);
assert_eq!(m[(3, 1)], 4);
assert_eq!(m[(3, 2)], 5);
assert_eq!(m[(3, 4)], 9);

assert_eq!(m[(4, 0)], 6);
assert_eq!(m[(4, 1)], 7);
assert_eq!(m[(4, 2)], 8);
assert_eq!(m[(4, 3)], 9);
```
