
fn main()
{
    println!("{}",f1(-123,45));
}


fn f1(x:i32 , y:i32) ->i32    //i->interger    ,u -> unsigned interger       , "->"  -> return
{
    println!("{} , {}",x,y);
    return x+y;
}