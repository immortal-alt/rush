pub fn say(cmd: &[String]) {
    let mut result: String = String::new();
    for (i, to_say) in cmd[1..].iter().enumerate() {
        if i > 0 { result.push(' '); }
        result.push_str(to_say);
    }
    println!("{}", result);
}

pub fn cat() {
    let cat = r#"    /\_____/\
   /  o   o  \
  ( ==  ^  == ) 
   )         (
  (           )
 ( (  )   (  ) )
(__(__)___(__)__)"#;
    
    println!("{}", cat);
}