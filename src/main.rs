use std::option::Option::None;

fn main() {
    let arr = [1, 3, 5, 7, 9];
  //  let result=sumOperation(&arr);  //原来写的方法

    let result=sum_u32(&arr);  //原来写的方法
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
//checked_add(): 时长的加法运算，超出Duration范围时返回None
//  pub const fn checked_add(self, rhs: u32) -> Option<u32>
// https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add  文档地址
//Checked integer addition. Computes self + rhs, returning None if overflow occurred.
#[allow(dead_code)]
fn sum_u32(a: &[u32]) -> Option<u32>{
    let mut sum: u32 = 0;
    for v in a.iter() {
        println!("当前累加的值为:{}",v);
        match sum.checked_add(*v) {
            //超出范围则返回NONE
            Some(v) =>{
                println!("求和前的值为:{},求和后的值为:{}",sum,v);
                sum = v
            }
            None => return None,
        }
    }
    Some(sum)
}






