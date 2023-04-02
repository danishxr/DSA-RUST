# Selection Sort using RUST
---

## Core Concept
---

> We begin with the unsorted array and then select the lowest element in the array and place the element in the appropriate posistion. We do this for all the elements in the array to get it sorted. 

> * selection sort is useful when the array size is small to be sorted or where mory space is limited. 
> * Overall selection sort is not the efficeint sorting algorithm and should be used wisely in performance critical applications.

---

### Lets understand this algorithm by an example 
---
We have our initial array 

`a=[1, 2, 8, 5, 6, 4, 3, 7]`


We initialise our variable  `i` for iterating through the elements in our array. We would be needing to check the current selected element for that we have variable `position`. To check current element with the next element we have `j=i+1`.

The second loop is to check from the `i+1` position till the end of the array. Basically to find the smallest element in the array from `i+1` to length of the array `len(a)`.


The condition which checks `a[position]` which is actually the current element with the next elements `a[j]` itertively.

Now if the current element is greater than the next element.

We know that the next element at position `j` is small. 

So we take its position and make it the current position `position`.



***We iterate like this till end of the array to find the smallest element.***

```
        a=[1, 2, 8, 5, 6, 4, 3, 7]
                 ^           ^
                 |           |
          -------|~~~~~~~~~~~|
                   ------>
                    swap
                   <------
                 

          ~~~~~~~~~~> compare in this direction

```

Now we need to swap the smallest to the inital position `i`. 

We have the most common swapping logic written. 

Store the `a[i]` value to a `temp` variable. Now we can put the smallest value to `a[i]`. 

And the first element which is in the variable `temp` is shifted to the location where the previous smallest position existed `a[position]`.

Now we increment `i` to go for the next element in the array.

This loop repeats for the second element position `i` and the next element `i+1` again to repeat the same process untill the array gets sorted.

---