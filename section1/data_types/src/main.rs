
#[derive(Debug)]
struct Foo {
    value : u32
}

fn reverse_and_print(foo : &mut Vec<Foo>){
    foo.reverse();
    for f in foo.iter(){
        println!("{:?}", foo);
    }
}

fn main() {

    let mut vector = vec!( Foo{value:1} , Foo{value:2} , Foo{value:3} );
    reverse_and_print(&mut vector)

    // let mut vec1 = vec![1,2,3];

    // vec1.push(4);
    // println!("{:?}",vec1);
    // vec1.pop();

    // println!("{:?}",vec1);

    // let mut vec2 = Vec::new();
    // vec2.push("test");
    // vec2.push("string");
    // println!("{:?}" , vec2);

    // vec2.reverse();
    // println!("{:?}" , vec2);

    // let mut vec3 = Vec::<i32>::with_capacity(2);
    // println!("{}", vec3.capacity());


    // let vec4: Vec<i32> = (1..5).collect();
    // println!("{:?}",vec4)
}
