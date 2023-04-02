///     DSA sorting algorithms: Selction sort
///     Author: Danish Xavier
///     email: danishxr@gmail.com
///     Twitter: @SillyTechy

/// Selection sort example, This is a stable sorting algorithm  
fn main() {
  
    
    let  mut a = [1, 2, 8, 5, 6, 4, 3, 7];
    let (mut i, mut j, mut position,mut temp);
    i = 0;
   
    while i < a.len() {

        // store position of the smallest number index
        position = i;
        
        j = i + 1;

        while j < a.len() {
            // find the position of the smallest element
            if a[position] > a[j] {
                position = j;
            }

            j +=1;
        }

        // Swap with the smallest element
        temp = a[i];
        a[i] = a[position];
        a[position] = temp;

        i +=1
    }
    println!("print sorted array  {:?}", a);
}