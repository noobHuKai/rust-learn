//! # 选择排序
//! ## 算法原理
//! 首先在未排序序列中找到最小（大）元素，存放到排序序列的起始位置，然后，再从剩余未排序元素中继续寻找最小（大）元素，然后放到已排序序列的末尾。以此类推，直到所有元素均排序完毕。
//! ## 算法描述
//! * 初始状态：无序区为R[1..n]，有序区为空；
//! * 第i趟排序(i=1,2,3…n-1)开始时，当前有序区和无序区分别为R[1..i-1]和R(i..n）。该趟排序从当前无序区中-选出关键字最小的记录 R[\k]，将它与无序区的第1个记录R交换，使R[1..i]和R[i+1..n)分别变为记录个数增加1个的新有序区和记录个数减少1个的新无序区；
//! * n-1趟结束，数组有序化了。
//! ## 动画演示
//! ![selectSort](https://images2017.cnblogs.com/blog/849589/201710/849589-20171015224719590-1433219824.gif)

fn selection_sort<T: Ord>(arr: &mut Vec<T>) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(min_index, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![3, 5, 6, 3, 1, 4];
        selection_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn reverse() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        selection_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
