use std::io::{self,stdin,Write};

pub fn print(){}

pub fn input(message:&str)->String{
    print!("{message}");
    io::stdout().flush().unwrap();
    let mut result:String=String::new();
    io::stdin().read_line(&mut result).unwrap(); 
    result
}

pub fn int(str:&str)->i128{
    let mut str:&str=str.trim();
    if str.is_ascii(){
        let mut result:i128=0;
        let mut rcl:i128=1;
        if str.chars().nth(0).unwrap()=='-'{rcl=-1;str=str.split_at(1).1}
        
        for digit in str.chars(){
            match digit as u8{
                b'0'=>result+=0,
                b'1'=>result+=1*rcl,
                b'2'=>result+=2*rcl,
                b'3'=>result+=3*rcl,
                b'4'=>result+=4*rcl,
                b'5'=>result+=5*rcl,
                b'6'=>result+=6*rcl,
                b'7'=>result+=7*rcl,
                b'8'=>result+=8*rcl,
                b'9'=>result+=9*rcl,
                _ =>panic!("[!] ILLEGAL ASCII CHARACTER {}",digit)
            }
            result*=10
        }
        return result/10
    }
    panic!("[!] NON-ASCII PROHIBITED")
}

pub fn float(){}

pub fn str(){}