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
fn bubble_sort(arr: &mut [i32]) {
    for i in 0..(arr.len() - 1) {
        for j in 0..(arr.len() - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut arr = [66, 18, 54, 67, 36, 44, 78, 18, 12, 56];
    println!("before : {:?}", arr);
    bubble_sort(& mut arr);
    println!("after : {:?}", arr);
}