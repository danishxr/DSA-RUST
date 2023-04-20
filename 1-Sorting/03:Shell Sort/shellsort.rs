use num::integer::div_floor;

fn swap(a: &mut [i8], p: usize, q: usize) {
    let temp = a[q];
    a[q] = a[p];
    a[p] = temp;
}

fn main() {
    let mut a = [35, 33, 42, 10, 14, 19, 27, 44];
    let mut gap = div_floor(a.len(), 2);

    while gap >= 1 {
        let mut i = 0;

        while gap + i < a.len() {
            if a[i] > a[gap + i] {
                swap(&mut a, i, gap + i);
                let mut p = i;

                while p >= gap {
                    if a[p - gap] > a[p] {
                        swap(&mut a, p - gap, p);
                        p -= gap;
                    } else {
                        break;
                    }
                }
            }

            i += 1;
        }

        gap = div_floor(gap, 2);
    }

    println!("The sorted array {:?}", a);
}
