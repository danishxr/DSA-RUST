///     DSA sorting algorithms: Selction sort
///     Author: Danish Xavier
///     email: danishxr@gmail.com
///     Twitter: @SillyTechy


/// Insertion sort example, This is a stable sorting algorithm  
fn main() {
    let mut a = [1, 2, 8, 5, 6, 4, 3, 7];
    let mut i:i32 = 0;
    let mut p;
    
    while i < (a.len() as i32 - 1 ){
        // Traverse the array to compare the values with the inital value
        if a[i as usize] > a[ i as usize + 1] {
            p = i;
    
            while p >= 0 {
                // Traverse to the left to insert the element in the right position
                if a[p as usize] > a [p as usize + 1] {
    
                    // Swap logic
                    let temp = a[p as usize + 1];
                    a[p as usize + 1] = a[p as usize];
                    a[p as usize] = temp;
    
                }
                // Specific condition to exit
                if p==0{
                    break;
                }
                else {
                     p -=1;
                }
            }
        }
        i +=1;
    
    }
    println!("Sorted array {:?}", a)
    }