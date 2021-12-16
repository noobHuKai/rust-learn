//! # 归并排序
//! ## 算法原理
//! 将已有序的子序列合并，得到完全有序的序列；即先使每个子序列有序，再使子序列段间有序。若将两个有序表合并成一个有序表
//! ## 算法描述
//! * 把长度为n的输入序列分成两个长度为n/2的子序列；
//! * 对这两个子序列分别采用归并排序；
//! * 将两个排序好的子序列合并成一个最终的排序序列。
//! ![mergeSort](https://images2017.cnblogs.com/blog/849589/201710/849589-20171015230557043-37375010.gif)

fn merge_sort(arr: &mut [i32]) {
    let mut temp = [0; 100];
    sort(arr, 0, (arr.len() - 1) as usize, &mut temp);
}

fn sort(arr: &mut [i32], left: usize, right: usize, temp: &mut [i32]) {
    if left < right {
        let mid = ((left + right) / 2) as usize;
        sort(arr, left, mid, temp); //左边归并排序，使得左子序列有序
        sort(arr, mid + 1, right, temp); //右边归并排序，使得右子序列有序
        merge(arr, left, mid, right, temp); //将两个有序子数组合并操作
    }
}
fn merge<'a>(arr: &'a mut [i32], mut left: usize, mid: usize, right: usize, temp: &'a mut [i32]) {
    let mut i = left;
    let mut j = mid + 1;
    let mut cur = 0;
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

#[test]
fn test_merge_sort() {
    let mut arr = [66, 18, 54, 67, 36, 44, 78, 18, 12, 56];
    println!("before : {:?}", arr);
    merge_sort(&mut arr);
    println!("after : {:?}", arr);
}