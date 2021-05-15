use std::usize;

fn nqueen(mut nq: i8, coor: &mut [i8], n: i8) {
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

        // coor[(nq+n) as usize] = y;
        // nq += 1;

        // for i in 0..nq {
        //     if coor[(i+n) as usize] == y || (nq-coor[i as usize])==(y-coor[(i+n) as usize]) || (nq-coor[i as usize])==-(y-coor[(i+n) as usize]) {
        //         flag = false;
        //     }
        // }

        // nqueen(nq, coor, n);
        
        // flag = true;
        // coor[(nq+n) as usize] = 0;
    }
}

pub fn run() {
    let mut f_coor:[i8; 15] = [-1; 15];
    let mut ans:[i8; 15] = [0; 15];
    // ans = nqueen(0, &mut f_coor, 5);
    nqueen(0, &mut f_coor, 5);
}