fn main()
{
//________________________________________Memory safety___________________________________________________________________________________________________

    //1 -> give error   
    let  num;
    println!("{}"num);


    //2 -> still give error  -> wait BUT why ??????  -> Rust is very Intelegent ha ha 
    if (true)
    {
        num =3;
    }
    println!("{}",num);


    //3 -> error resorlved
    if (true)
    {
        num =3;
    }
    else
    {
        num =4
    }
    println!("{}",num);
}