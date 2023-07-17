
//fn validate_grid(g : i128) -> bool 

fn popcount(g : i128) -> i128{
   let mut n = g;
    let mut sum = 0;
    while n != 0 {
        sum = sum + (n % 2);
        n = n / 2;
    }
    return sum;
}

fn print_grid(g : i128) -> (){
    let mut gr = g;
    for _ in 0..9 {
        let mut s = String::new();
        for _ in 0..9 {
            let b = gr % 2;
            if b == 1 {
                s.push('#');
            }
            else {
                s.push('.');
            }
            gr = gr / 2;
        }
        println!("{}",s);
    }
}



fn square_mask(n : i32)  -> i128 {
    let og_mask = 0b111000000111000000111;
    let x = n % 3;
    let y = n / 3;
    return og_mask << ((x * 3 + y * 3* 9));
}

fn line_mask(n: i32) -> i128 {
    let og_mask = 0b111111111;
    return og_mask << (n * 9);
}

fn column_mask(n: i32) -> i128 {
    let og_mask = 0b100000000100000000100000000100000000100000000100000000100000000100000000100000000100000000 >> 8;
    return og_mask << n;
}

fn check_partial_grid(g:i128) -> bool {
    for i in 0..9 {
        let n1 = popcount(g & square_mask(i));
        let n2 =popcount(g & line_mask(i));
        let n3 =popcount(g & column_mask(i));
        if n1 > 1 || n2 > 1 || n3 > 1 {return false;}
    }
    true
}

fn check_complete_grid(gr : Complete_grid) -> bool {
    //assuming there is no overlap between grids
    check_partial_grid(gr.grid0) && check_partial_grid(gr.grid1) && check_partial_grid(gr.grid2) && check_partial_grid(gr.grid3) && check_partial_grid(gr.grid4) && check_partial_grid(gr.grid5) && check_partial_grid(gr.grid6) && check_partial_grid(gr.grid7) && check_partial_grid(gr.grid8) && check_partial_grid(gr.grid9)  
}


struct Complete_grid {
    comp : i128,
    grid0 : i128,
    grid1 : i128,
    grid2 : i128,
    grid3 : i128,
    grid4 : i128,
    grid5 : i128,
    grid6 : i128,
    grid7 : i128,
    grid8 : i128,
    grid9 : i128,
}


fn main() {
    let n = popcount(542);
    print_grid(column_mask(2));
    println!("Hello, {}",n);
}
