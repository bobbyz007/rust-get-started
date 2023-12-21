pub fn iterator_adapter() {
    let arr = [1, 2, 3, 4, 5, 6];
    let sum = arr.iter().step(2).fold(0, |acc, x| acc + x);
    println!("iterator sum: {:?}", sum);
}

struct Step<I> {
    iter: I,
    skip: usize,
}

impl<I: Iterator> Iterator for Step<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let elt = self.iter.next();
        if self.skip > 0 {
            // 消费当前item，后续调用next会排除当前item从而返回下一个item
            self.iter.nth(self.skip - 1);
        }
        elt
    }
}

// 生成step适配器
fn step<I: Iterator>(iter: I, step: usize) -> Step<I> {
    Step {
        iter,
        skip: step - 1,
    }
}

trait IterExt: Iterator {
    fn step(self, n: usize) -> Step<Self>
    where Self: Sized
    {
        step(self, n)
    }
}
// 为所有的迭代器 都实现step方法
impl<T: ?Sized + Iterator> IterExt for T {}