# Insertion Sort using RUST

---

## Core Concept

---

> This is a simple sorting algorithm which works by iteratively building a sorted sublist within an unsorted list. It inserts the elements from the unsorted sublist to the right position in the sorted sublist. Untill the entire list becomes sorted.First we declare an array `a` to be sorted.

> - It is best to be used when the sorting items who number of items is less

- It is also works well where you have an already sorted item and you want to add new items to the sorted array
- ***

### Lets understand this algorithm by an example

The main logic of insertion sort is we would be having an unsorted array, for example:

`a=[1, 2, 8, 5, 6, 4, 3, 7]`

This array `a` is mutable as we have to replace values in the array during the swaps.

Now for iteration we need two variable `i` and `p` and both are mutable.

Now the sorting works in this way.

The first character in iteself not has to be sorted.
The next character `a[i as usize + 1]` we take in and

> Array indexing or slice indices are of type `usize` so we need to convert our `i32` type to `usize` type.
> Compare it with the first character `a[i as usize]`.

Now we iterate till `a[i as usize ] > a[i as usize + 1]` becomes `True`

```
	[1, 2, 8, 5, 6, 4, 3, 7]
		   ^  ^
		   |  |
	-------|~~|

     ~~~~~~~> compare in this direction
```

, here in the example it is `8 > 5`.

We need to swap the Values here inorder to sort it
but before doing that we will save the position in the vairable called `p`.

Now we will go ahead and swap the position.
we create a temporary immutable variable `temp` for simple swap logic.
Now the array will be like this.

```

	[1, 2, 5, 8, 6, 4, 3, 7]

```

Now once swap is made we need to check if the previous values are smaller than the swapped value.

So we have to iterate again in the opposite direction. That is why we are having another while loop which goes till the starting postion `0`. And `p = p-1` this will decrement and we again compare in the opposite direction.

```

	[1, 2, 5, 8, 6, 4, 3, 7]
		^  ^
		|  |
	----|~~|

	<------ compare in this direction
```

If a value is found to be greater than the current swapped value, We swap again and this process is repeated, till the postion of 0 is checked.

Now we come to the outer loop where the `i` holds the location where the initial swap started then this is again continued forward to check if any values are greater.

This process is repeated untill every values is checked and sorted.

The while loop is happening twice with the swapping, so in the worst case Time complexity becomes `O(N^2)`.

---
