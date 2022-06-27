//! # 冒泡排序
//! ## 算法原理
//! 它重复地走访过要排序的数列，一次比较两个元素，如果它们的顺序错误就把它们交换过来。走访数列的工作是重复地进行直到没有再需要交换，也就是说该数列已经排序完成
//! ## 算法描述
//! * 比较相邻的元素。如果第一个比第二个大，就交换它们两个；
//! * 对每一对相邻元素作同样的工作，从开始第一对到结尾的最后一对，这样在最后的元素应该会是最大的数；
//! * 针对所有的元素重复以上的步骤，除了最后一个；
//! * 重复步骤1~3，直到排序完成。
//! ## 动图演示
//! ![bubbleSort](https://images2017.cnblogs.com/blog/849589/201710/849589-20171015223238449-2146169197.gif)

// 冒泡排序
fn bubble_sort<T: Ord>(arr: &mut Vec<T>) {
    if arr.is_empty() {
        return;
    }
    for i in 0..(arr.len() - 1) {
        for j in 0..(arr.len() - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![3, 5, 6, 3, 1, 4];
        bubble_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn reverse() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
