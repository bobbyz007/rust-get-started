use std::cmp::Ordering;

/// PartialEq, Eq, PartialOrd, Ord, Hash
///
/// 1. 如果实现了PartialEq，尽可能也实现Eq（等价性）， Eq继承PartialEq只是一个标签trait，没有定义方法。
///    一个例外 浮点数只能实现PartialEq，而不能实现Eq（自反性，对称性，传递性），因为IEEE标准规定NaN不能作比较。
///    如果两个元素具有相等关系,那它们之间也一定有部分相等关系。
///
/// 2. 用于排序的Ord（全序，cmp方法），PartialOrd（偏序，partial_cmp方法，不具备完全性）： 在你为自定义类型实现PartialOrd之前，你必须首先为其实现PartialEq。
///    2.1 partial_cmp方法 返回 Option<Ordering>， 因为浮点数的NaN不能进行比较，返回None。
///    2.2 cmp方法 返回Ordering， 有明确的比较结果。因此若要自定义类型可排序，必须为它实现Ord 。
///    2.3 全序关系必然是偏序关系，在你实现Ord 之前， 你首先必须实现PartialOrd, Eq, PartialEq。
///
/// 3. HashMap中的key 必须实现 Eq，所以浮点数不能作为key。

// 原生数字类型和字符串已实现上述trait
pub fn cmp_order() {
    // 浮点数只能partial_cmp
    let result = 1.0.partial_cmp(&2.0);
    assert_eq!(result, Some(Ordering::Less));

    let result = 1.partial_cmp(&2);
    assert_eq!(result, Some(Ordering::Less));
    let result = 1.cmp(&2);
    assert_eq!(result, Ordering::Less);

    let result = "abc".partial_cmp("Abc");
    assert_eq!(result, Some(Ordering::Greater));
    let result = "abc".cmp(&"Abc");
    assert_eq!(result, Ordering::Greater);
}