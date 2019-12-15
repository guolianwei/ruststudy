fn borrow() {
    //Don't worry if you don't understand how `fold` works ,
    //the point here is that an immutable reference is borrowed.
    fn sum_vec(v:&Vec<i32>) ->i32 {
         v.iter().fold(0,|a,&b | a +b)
    }
}
//reference 和borrow有什么区别
//
//reference 和borrow其实是一个含义，但是被拆分成两个单词；
// borrow是指函数运行必备的参数变量，需要依赖外部提供，这种参数声明就叫做borrow（借用）；
fn reference_example(){
    let a=String::from("reference hello");
    let b= &a;  // 这是引用
    println!("a:{},b{}",a,b );
    
}
fn borrow_example(){
    let a=String::from("borrow hello");
    fn print(msg: &String){
        println!("{:?}", msg);
    }
    print(&a);
}