use std::env;
use std::usize;

fn nqueen(nq: i8, coor: &mut [i8], n: i8) {
    if nq == n {
        // println!("{:?}", coor);

        for x in 0..n {
            for y in 0..n {
                if y == coor[x as usize] {
                    print!("x ");
                } else {
                    print!("o ");
                }
            }
            print!("\n");
        }
        print!("\n");
        return;
    }

    let mut flag: bool = true;

    for y in 0..n {
        for i in 0..nq {
            if coor[i as usize] == y || (nq-i)==(y-coor[i as usize]) || (nq-i)==-(y-coor[i as usize]) {
                flag = false;
                break;
            }
        }

        if flag {
            coor[nq as usize] = y;
            nqueen(nq+1, coor, n); 
            coor[nq as usize] = -1;
        }

        flag = true;
    }
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[2].clone();
    let n: i8 = command.parse::<i8>().unwrap();
    let mut f_coor:[i8; 15] = [-1; 15];

    nqueen(0, &mut f_coor, n);

    if n <= 3 {
        println!("\n\n\n============================= NONE =============================\n\n\n");
    }
}