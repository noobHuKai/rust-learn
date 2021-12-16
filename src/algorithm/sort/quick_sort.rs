//! # 快速排序
//! ## 算法原理
//! 通过一趟排序将待排记录分隔成独立的两部分，其中一部分记录的关键字均比另一部分的关键字小，则可分别对这两部分记录继续进行排序，以达到整个序列有序
//! ## 算法描述
//! * 从数列中挑出一个元素，称为 “基准”（pivot）；
//! * 重新排序数列，所有元素比基准值小的摆放在基准前面，所有元素比基准值大的摆在基准的后面（相同的数可以到任一边）。在这个分区退出之后，该基准就处于数列的中间位置。这个称为分区（partition）操作；
//! * 递归地（recursive）把小于基准值元素的子数列和大于基准值元素的子数列排序。
//! ## 动画演示
//! ![quickSort](https://images2017.cnblogs.com/blog/849589/201710/849589-20171015230936371-1413523412.gif)

fn quick_sort(arr: &mut [i32]) {
    sort(arr, 0, arr.len() - 1);
}

fn sort(arr: &mut [i32], left: usize, right: usize) {
    if left >= right {
        return;
    }
    let mut i = left;
    let mut j = right;
    let temp = arr[left];

    while i != j {
        while i < j && arr[j] >= temp {
            j -= 1;
        }
        while i < j && arr[i] <= temp {
            i += 1;
        }
        if i < j {
            arr.swap(i, j);
        }
    }
    arr.swap(i, left);
    if i == 0{
        return;
    }
    sort(arr, left, i - 1);
    sort(arr, j + 1, right)
}

#[test]
fn test_quick_sort() {
    let mut arr = [66, 18, 54, 67, 36, 44, 78, 18, 12, 56];
    println!("before : {:?}", arr);
    quick_sort(&mut arr);
    println!("after : {:?}", arr);
}
