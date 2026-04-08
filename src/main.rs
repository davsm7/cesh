//CESH - César's Extensible SHell | indev 0.0.1
//Una implementación de una shell sencilla sin bloat y extensible
//César David Amezcua Naranjo

//                 _    
//       __ ___ __| |_  
//      / _/ -_|_-< ' \ 
//      \__\___/__/_||_|

use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    loop{

        print!("cesh > ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut partes = input.trim().split_whitespace();

        let comando = match partes.next(){
            Some(c) => c,
            None => continue,
        };
        let args = partes;

        match comando{
            "cd" => {
                let nuevo_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(nuevo_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("cesh: {}", e);
                }
            }
            "exit" => return,
            comando => {
                let child = Command::new(comando)
                .args(args)
                .spawn();

                match child{
                    Ok(mut child) => {
                        let _ = child.wait();
                    }
                    Err(e) => eprintln!("cesh: {}", e),
                }
            }
        }
    }
}
