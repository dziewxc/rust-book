fn main() {
    println!("Hello, world!");

    let s1 = "literal"; // literal, immutable, fixed memory
    //literal is stored on stack

    let mut s2 = String::from("string"); //string, mutable
    //stored on heap

    s2.push_str("version2");

    let s3 = String::from("example");

    let x = s3; //value of s3 was burrowed, we can't refer to s3 anymore

    //when we would free memory for s3 and x after function ended, it would be double free

    //println!("value of s3: {}", s3); //we can't refer to s3 anymore
    //let x = s3.push_str("ff"); //we can't do that neither

    let i1 = 1;
    let i2 = i1; //this is copy, we can change both values separately
    println!("i1 value: {}", i1);
    println!("i2 value: {}", i2); //this works because size is fixed and they are both on stack

    let u1 = String::from("promotion");
    check_something(u1); //the function check_something has now ownership of u1

    //println!("{}", u1); //error

    let u2 = 1;
    check_something_on_stack(u2); //we are copying integer because it's on stack
    println!("still here, {}", u2); // we can still use it

    let b2 = returning_something();
    println!("Hi, I took ownership, {}", b2);

    let c1 = String::from("my ownership will be gave back");

    let c2 = take_ownership_and_return(c1);

    //println!("c1: {}", c1); //we can't access c1 here, it was moved
    //we gave c1 ownership, so we don't have to drop it
    //c2 will be dropped after end of the scope

    //the value will be cleaned up by drop unless the data has been moved to be owned by another variable
}

fn check_something(u1:String)
{
    println!("I'm here now: {}!", u1);
} //after this function is finished, we are calling drop() because it's on heap

fn check_something_on_stack(u2:i32)
{
    println!("Yo, {}", u2);
} //u2 is Copy, so we don't call drop() here

fn returning_something() -> String
{
    String::from("sth sth")
}

fn take_ownership_and_return(c1:String) -> String //c1 moved here and then ownerships is going back to c2
{
    c1
}