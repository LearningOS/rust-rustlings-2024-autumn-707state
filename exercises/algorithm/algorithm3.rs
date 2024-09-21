/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Clone + std::cmp::PartialOrd>(array: &mut [T]) {
    let length = array.len();
    for i in 0..length {
        // 设置一个标志，表示是否有交换发生
        let mut swapped = false;

        for j in 0..length - 1 - i {
            if array[j] > array[j + 1] {
                // 交换元素
                array.swap(j, j + 1);
                swapped = true;
            }
        }

        // 如果没有发生交换，数组已经排序好，可以提前退出
        if !swapped {
            break;
        }
    }
    // let length = array.len();
    // for i in 1..length {
    //     let key = array[i].clone(); // Clone the key element
    //     let mut j = i as isize - 1; // Start comparing with the previous element

    //     // Move elements that are greater than the key to one position ahead of their current position
    //     while j >= 0 && array[j as usize] > key {
    //         array[j as usize + 1] = array[j as usize].clone();
    //         j -= 1;
    //     }
    //     array[j as usize + 1] = key; // Place the key at its correct position
    // }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
