//! # 插入排序
//! ## 算法原理
//! 通过构建有序序列，对于未排序数据，在已排序序列中从后向前扫描，找到相应位置并插入。
//! ## 算法描述
//! * 从第一个元素开始，该元素可以认为已经被排序；
//! * 取出下一个元素，在已经排序的元素序列中从后向前扫描；
//! * 如果该元素（已排序）大于新元素，将该元素移到下一位置；
//! * 重复步骤3，直到找到已排序的元素小于或者等于新元素的位置；
//! * 将新元素插入到该位置后；
//! * 重复步骤2~5。
//! ## 动画演示
//! ![insertSort](https://images2017.cnblogs.com/blog/849589/201710/849589-20171015225645277-1151100000.gif)

fn insertion_sort<T: Ord + Copy>(arr: &mut Vec<T>) {
    //赋值交换
    for i in 1..arr.len() {
        let mut prev_index: i32 = i as i32 - 1;
        let cur = arr[i];
        while (prev_index >= 0) && (arr[prev_index as usize] > cur) {
            arr[(prev_index + 1) as usize] = arr[prev_index as usize];
            prev_index -= 1; //不用 usize的原因是这里可能为-1
        }
        arr[(prev_index + 1) as usize] = cur;
    }
}

fn insertion_sort_swap<T: Ord + Copy>(arr: &mut Vec<T>) {
    //直接交换
    for i in 1..arr.len() {
        let mut j = i;
        while ((j as i32) - 1 >= 0) && arr[j] < arr[j - 1] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![66, 18, 54, 67, 36, 44, 78, 18, 12, 56];
        insertion_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1])
        }
    }

    #[test]
    fn test_insertion_sort1() {
        let mut arr = vec![66, 18, 54, 67, 36, 44, 78, 18, 12, 56];
        insertion_sort_swap(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1])
        }
    }
    #[test]
    fn basic() {
        let mut arr = vec![3, 5, 6, 3, 1, 4];
        insertion_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn reverse() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        insertion_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
