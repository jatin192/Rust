fn main() 
{

//____________________________________mut_______________________________________________________________________________________________________________
    let a =2;             // by default varibale is immutable    ->    Safety
                         // a= a+4;  -> error
    
    let mut b =3;      // mutable variable using "mut" keyword
    b = b+2;

    
    println!("{}",a);                                // understanding -> printf("%d",a);
    println!("{}",b);
}