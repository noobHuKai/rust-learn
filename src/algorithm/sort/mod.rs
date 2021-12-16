//! # 排序算法
//! [原文连接](https://www.cnblogs.com/onepixel/articles/7674659.html)
//!
//!
//! ![sort](https://images2018.cnblogs.com/blog/849589/201804/849589-20180402133438219-1946132192.png)


//! 常用结构会实现 sort 方法：
//! 1. sort()  稳定排序（不会破坏相同值的顺序）
//! 2. sort_unstable()  不稳定排序
pub mod bubble_sort;
pub mod selection_sort;
pub mod insertion_sort;
pub mod shell_sort;
pub mod merge_sort;
pub mod quick_sort;


#[test]

fn test_sort() {
    let mut a = vec![66, 18, 54, 67, 36, 44, 78, 18, 12, 56];
    a.sort();
    println!("{:?}", a);

    let mut b = [66, 18, 54, 67, 36, 44, 78, 18, 12, 56];
    b.sort();
    println!("{:?}", b);

}