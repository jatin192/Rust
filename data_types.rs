fn main()
{
    

    const A_1:f64 = 5.4;     // const var name must be in CAPS 
                            //  const > let  (fast compile)

                    

    let mut a:u8 =10;     /*    
              u16
              u32
              u64          */

     

    let mut b:f32 =3.14;
    //    f64  


    let mut c:bool = false;


    let mut d:&str = "jatin";

    let mut arr = [1,2,3];  
    arr[2] =77;



    let mut tupple_ = (1,"jatin",3.4);
    tupple_.1 = "akshay";




    println!("{},{},{},{},{},{},{}",A_1,a,b,c,d,arr[2],tupple_.1);


   /* array -> hold homogeneous value
    tupple -> hold hetrogeneous value
    */
    
}






//  i8
//  i16
//  i32
//  i64