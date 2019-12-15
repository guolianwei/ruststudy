trait Interface {
    fn exposed(&self, a: &str) -> bool;

}
struct Implementation {
    value: i32,
    has: bool,
}
impl Interface for Implementation {
    fn exposed(&self, a: &str) -> bool {
        if self.value == 0 {
            true
        } else {
            false
        }
    }
}

struct Point{
    x:i32,
    y:i32,
}

struct PointRef<'a> {
    x:&'a mut i32,
    y:&'a mut i32,
}
fn name()  {
    let mut orgin=Point{x:0,y:0};
    {
        let r=PointRef {x:&mut orgin.x,y:&mut orgin.y};
        *r.x=5;
        *r.y=6;
        assert_eq!(5,orgin.x );
        assert_eq!(6,orgin.y );
        
    }
    orgin.x=0;
    let orgin=orgin; //orgin is now immutable
    println!("The orgin is at {},{}",orgin.x,orgin.y);
}