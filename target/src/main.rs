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
                pattern: Vec<(isize, isize)>
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
    gen_grid_struct!(3, 3, u64);
    let mut gr = Grid{
        data: tab,
        default: 0
    };
    println!("{:?}", gr);

    let pat = vec![
        (-1, -1),
        ( 0, -1),
        ( 1,  0),
        ( 1,  1),
        ( 0,  1),
        (-1,  1),
        (-1,  0)
    ];
    
    for i in -2..5
    {
        for j in -1..6
        {
            print!("{} ", gr.get_cell(i, j));
        }
        println!();
    }


    println!("{:?}", gr.get_cells((1,0), pat.clone()));
    let manuel = vec![1,1,1,1,1,1,1,1];

    for i in -2..5
    {
        for j in -1..6
        {
            print!("{} ", gr.get_cell(i, j));
        }
        println!();
    }

    gr.set_cells((1, 0), pat.clone(), manuel);

    for i in -2..5
    {
        for j in -1..6
        {
            print!("{} ", gr.get_cell(i, j));
        }
        println!();
    }

    println!("{:?}", gr.get_cells((1,0), pat));
    
}
