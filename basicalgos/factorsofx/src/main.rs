

fn main() {
    println!("Finding factors of x!");

	let x = 9;
	let mut mul = 1;
    let mut done = false;
    let mut j = 0;
    while !done{
        if (x%mul == 0) && !(j == mul)
        {
            j = x/mul;
            println!("{}",mul );
            println!("{}",j );
            mul = mul + 1;

        }
        else if j == mul
        {
            done = true;
        }
        else{
            mul = mul + 1;
        }

    }



}
