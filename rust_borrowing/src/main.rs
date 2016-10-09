fn main() {
    let mut name = format!("Hello World! ");
 
    name.push('a');
  
    {
        //   let r : &String = &name;    // borrowing as immutable
        let r : &mut String = &mut name;    // borrowing as mutable
        r.push('b');  //can't do that, borrowed as immutable
    }
  
    printer(&name); // borrowing as immutable
    
    printer_mut(&mut name); // borrowing as mutable
}

fn printer(txt: &String) {
    println!("{}", txt);
}

fn printer_mut(txt: &mut String) {
    txt.push('c');
    println!("{}", txt);
}
