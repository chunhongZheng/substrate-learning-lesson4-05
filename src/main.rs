use std::option::Option::None;

fn main() {
    let arr = [1, 3, 5, 7, 9];
    let result=sumOperation(&arr);
    match result {
        Option::Some(something) => {
            println!("求和结果为:{}", something);
        },
        Option::None => {
            println!("求和异常");
        }
    }
}
//实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
fn sumOperation(x: &[u32])->Option<u32>{
    let sumResult=x.iter().sum::<u32>();
    return Some(sumResult);
}







