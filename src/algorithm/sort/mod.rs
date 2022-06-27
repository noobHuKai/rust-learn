//! # 排序算法
//! [原文连接](https://www.cnblogs.com/onepixel/articles/7674659.html)
//!
//!
//! ![sort](https://images2018.cnblogs.com/blog/849589/201804/849589-20180402133438219-1946132192.png)

//! 常用结构会实现 sort 方法：
//! 1. sort()  稳定排序（不会破坏相同值的顺序）
//! 2. sort_unstable()  不稳定排序
pub mod bubble_sort;
#[allow(rustdoc::broken_intra_doc_links)]
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;
pub mod shell_sort;

#[cfg(test)]
mod tests {
    #[test]
    fn test_std_sort() {
        let mut vec1 = vec![66, 18, 54, 67, 36, 44, 78, 18, 12, 56];
        vec1.sort();
        for i in 0..vec1.len() - 1 {
            assert!(vec1[i] <= vec1[i + 1])
        }

        let mut vec2 = [66, 18, 54, 67, 36, 44, 78, 18, 99, 56];
        vec2.sort_unstable();
        for i in 0..vec2.len() - 1 {
            assert!(vec2[i] <= vec2[i + 1])
        }
    }
}
