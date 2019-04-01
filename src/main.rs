/*
lib permettant de gérer des grilles 2D facilement
(pour automates cellulaires et autres)
 */

use std::fmt;

macro_rules! gen_grid_struct {
    ($w:expr,$h:expr, $t:ty) => (
        struct Grid
        {
            data: [[$t; $h]; $w],
            default: $t
        }

        impl fmt::Debug for Grid {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.data.fmt(f)
            }
}
        impl Grid
        {
            fn get_width(&self) -> usize
            {
                $w
            }
            fn get_height(&self) -> usize
            {
                $h
            }

            fn get_cell(&self, x: isize, y: isize) -> $t
            {
                if x < $w && y < $h && x >= 0 && y >= 0
                {
                    self.data[x as usize][y as usize].clone()
                }
                else
                {
                    self.default.clone()
                }
            }

            fn set_cell(&mut self, val: $t, x: isize, y: isize)
            {
                if x < $w && y < $h && x >= 0 && y >= 0
                {
                    self.data[x as usize][y as usize] = val;
                }
            }

            fn get_cell_tuple(&self, (x, y): (isize, isize)) -> $t
            {
                self.get_cell(x, y)
            }

            fn get_cells(
                &self,
                (x0, y0): (isize, isize),
                pattern: &Vec<(isize, isize)>
            ) -> Vec<$t>
            {
                let mut vec = Vec::new();
                for (x, y) in pattern.iter()
                {
                    vec.push(self.get_cell(x+x0, y+y0));
                }
                vec
            }

            fn set_cells(
                &mut self,
                (x0, y0):(isize, isize),
                pattern: Vec<(isize, isize)>,
                mut content: Vec<$t>
            )
            {
                for (x, y) in pattern.iter()
                {
                    let val = match content.pop()
                    {
                        Some(x) => x,
                        _       => self.default.clone()
                    };
                    self.set_cell(
                        val,
                        x+x0,
                        y+y0
                    );
                }
            }
            
        }
    )
}





fn main() {
    let mut tab = [[1 as u64,2,3],[4,5,6],[7,8,9]];
    println!("{:?}", tab);
    gen_grid_struct!(32, 32, u64);
    /*
    let mut gr = Grid{
        data: tab,
        default: 0
    };
    println!("{:?}", gr);

    
    for i in -2..5
    {
        for j in -1..6
        {
            print!("{} ", gr.get_cell(i, j));
        }
        println!();
    }

    let manuel = vec![1,1,1,1,1,1,1,1];

    for i in -2..5
    {
        for j in -1..6
        {
            print!("{} ", gr.get_cell(i, j));
        }
        println!();
    }


    for i in -2..5
    {
        for j in -1..6
        {
            print!("{} ", gr.get_cell(i, j));
        }
        println!();
    }

    println!("{:?}", gr.get_cells((1,0), &pat));

     */

    let pat = vec![
        (-1, -1),
        ( 0, -1),
        ( 1, -1),
        ( 1,  0),
        ( 1,  1),
        ( 0,  1),
        (-1,  1),
        (-1,  0)
    ];

    let mut tab = [[0u64; 32]; 32];

    tab[1][1] = 1;
    tab[2][2] = 1;
    tab[2][3] = 1;
    tab[1][3] = 1;
    tab[0][3] = 1;

    let mut grs = [
        Grid
        {
            data: tab,
            default: 0
        },
        Grid{
            data: tab,
            default: 0
        }
    ];
    
    let mut flag_n = 0usize;
    let mut flag_np1 = 1;
    let mut tmp = 0;
    let mut s = 0;
    loop
    {
        println!("flags:{} {}", flag_n, flag_np1);
        
        for i in 0..(grs[flag_n].get_width() as isize)
        {
            for j in 0..(grs[flag_n].get_height() as isize)
            {
                s = 0;
                for cell in grs[flag_n].get_cells((i, j), &pat)
                {
                    if cell == 1
                    {
                        s+=1;
                    }
                }
                
                grs[flag_np1].set_cell(
                    if s < 2 || s > 3
                {
                    0
                } else
                {
                    if s == 3
                    {
                        1
                    }
                    else
                    {
                        grs[flag_n].get_cell(i, j)
                    }
                }, i, j);
                print!("{}", grs[flag_n].get_cell(i, j));
            }
            println!();
        }


        tmp = flag_np1;
        flag_np1 = flag_n;
        flag_n = tmp;
            
    }
}
