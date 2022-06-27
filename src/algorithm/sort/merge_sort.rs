//! # 归并排序
//! ## 算法原理
//! 将已有序的子序列合并，得到完全有序的序列；即先使每个子序列有序，再使子序列段间有序。若将两个有序表合并成一个有序表
//! ## 算法描述
//! * 把长度为n的输入序列分成两个长度为n/2的子序列；
//! * 对这两个子序列分别采用归并排序；
//! * 将两个排序好的子序列合并成一个最终的排序序列。
//! ![mergeSort](https://images2017.cnblogs.com/blog/849589/201710/849589-20171015230557043-37375010.gif)

fn merge_sort<T: Ord + Copy>(arr: &mut Vec<T>) {
    if arr.is_empty() {
        return;
    }
    sort(arr, 0, (arr.len() - 1) as usize);
}

fn sort<T: Ord + Copy>(arr: &mut Vec<T>, left: usize, right: usize) {
    if left < right {
        let mid = ((left + right) / 2) as usize;
        sort(arr, left, mid); //左边归并排序，使得左子序列有序
        sort(arr, mid + 1, right); //右边归并排序，使得右子序列有序
        merge(arr, left, mid, right); //将两个有序子数组合并操作
    }
}
fn merge<'a, T: Ord + Copy>(
    arr: &'a mut Vec<T>,
    mut left: usize,
    mid: usize,
    right: usize,
) {
    let mut i = left;
    let mut j = mid + 1;
    let mut cur = 0;
    let mut temp = Vec::new();
    // todo:
    for _ in 0..2*right{
        temp.push(arr[i]);
    }
    while i <= mid && j <= right {
        if arr[i] <= arr[j] {
            temp[cur] = arr[i];
            i += 1;
        } else {
            temp[cur] = arr[j];
            j += 1;
        }
        cur += 1;
    }
    while i <= mid {
        //将左边剩余元素填充进temp中
        temp[cur] = arr[i];
        i += 1;
        cur += 1;
    }
    while j <= right {
        //将右边剩余元素填充进temp中
        temp[cur] = arr[j];
        j += 1;
        cur += 1;
    }
    cur = 0;
    //将temp中的元素全部拷贝到原数组中
    while left <= right {
        arr[left] = temp[cur];
        left += 1;
        cur += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![3, 5, 6, 3, 1, 4];
        merge_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn reverse() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        merge_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
