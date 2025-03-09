

fn sort<T>(array: &mut [T]) where T: PartialOrd {
    let n = array.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n-i-1 {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
                swapped=true;
            }
        }

        // 没有触发比较，说明已经是有序的了
        if !swapped  {
            break
        }
    }
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