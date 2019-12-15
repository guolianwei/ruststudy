fn name(arg: f32) -> i32 {
   let plus_one =| x:i32| x+1;
   plus_one(1);
   let plus_two= |x| {
       let mut result:i32 =x;
       result +=1;
       result +=1;
       result
   };
   let num =1;
   plus_two(2)
}
fn closures_and_their_environment(arg: String) -> i32 {
    let mut num1 = 5;
    let plus_num = |x:i32| x+num1;

    // let y = &mut num1;
    let nums=vec![1,2,3];
    let take_nums = || nums;
  

    num1
}
fn move_closures (){
    let mut num =5;
    {
        let mut add_num = move |x:i32|  num+=x;
        add_num(5);
    }
}
fn factory() ->  Box<Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}
fn factory1() -> Box<Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}



