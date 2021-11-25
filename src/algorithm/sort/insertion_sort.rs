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



fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut prev_index = i - 1;
        let cur = arr[i];
        while (prev_index >= 0) && (arr[prev_index] > cur) {
            arr[prev_index + 1] = arr[prev_index];
            prev_index -= 1;
        }
        arr[prev_index + 1] = cur;
    }
}


#[test]
fn test_insertion_sort() {
    let mut arr = [66, 18, 54, 67, 36, 44, 78, 18, 12, 56];
    println!("before : {:?}", arr);
    insertion_sort(&mut arr);
    println!("after : {:?}", arr);
}

