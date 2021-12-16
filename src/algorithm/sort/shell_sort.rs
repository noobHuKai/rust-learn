//! # 希尔排序
//! ## 算法原理
//! 简单插入排序的改进版,它通过比较相距一定间隔的元素来进行，各趟比较所用的距离随着算法的进行而减小，直到只比较相邻元素的最后一趟排序为止。
//! ## 算法描述
//! 先将整个待排序的记录序列分割成为若干子序列分别进行直接插入排序，具体算法描述：
//! * 选择一个增量序列t1，t2，…，tk，其中ti>tj，tk=1；
//! * 按增量序列个数k，对序列进行k 趟排序；
//! * 每趟排序，根据对应的增量ti，将待排序列分割成若干长度为m 的子序列，分别对各子表进行直接插入排序。仅增量因子为1 时，整个序列作为一个表来处理，表长度即为整个序列的长度。
//! ## 动画演示
//! ![shellSort](https://images2018.cnblogs.com/blog/849589/201803/849589-20180331170017421-364506073.gif)

fn shell_sort(arr: &mut [i32]) {
    let mut gap = arr.len() / 2;
    while gap > 0 {
        for i in gap..arr.len() {
            let mut j = i;
            while ((j as i32) - (gap as i32)) >= 0 && arr[i] < arr[j - gap] {
                arr.swap(j, j - gap);
                j = j - gap;
            }
        }
        gap = gap / 2;
    }
}

#[test]
fn test_shell_sort() {
    let mut arr = [66, 18, 54, 67, 36, 44, 78, 18, 12, 56];
    println!("before : {:?}", arr);
    shell_sort(&mut arr);
    println!("after : {:?}", arr);
}
