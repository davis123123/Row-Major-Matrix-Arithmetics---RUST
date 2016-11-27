use std::{ops, fmt};
//use std::ops::{Index, Add, Sub, Mul, Neg};
use std::string::String;
use std::vec::Vec;
use std::slice;
use std::convert::Into;
use std::vec;
use std::iter::Iterator;
#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        Matrix{row:row, col:col, data:values.to_vec()}
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        Matrix{row:row, col:col, data:Vec::new()}
    }//done

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }//done
    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }//done

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}


impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        if rhs.row != self.row || self.col != rhs.col{
            panic!();
        }

        else{
            let u: usize = self.row * self.col;
            let mut temp_vec: Vec<T> = Vec::new();
            for x in 0..u{
                temp_vec.push(self.data[x] + rhs.data[x]);
            }

            Matrix{
                row: self.row,
                col: self.col,
                data: temp_vec,
            }
        }
        /*let u: usize = self.row * self.col;
        let mut temp_vec: Vec<T> = Vec::new();
        for x in 0..u{
            temp_vec.push(self.data[x] + rhs.data[x]);
        }*/
        //return temp_vec;
        //unimplemented!();
    }//NOT done
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;
    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.row != self.row || self.col != rhs.col{
            panic!();
        }

        else{
            let u: usize = self.row * self.col;
            let mut temp_vec: Vec<T> = Vec::new();
            for x in 0..u{
                temp_vec.push(self.data[x] - rhs.data[x]);
            }

            Matrix{
                row: self.row,
                col: self.col,
                data: temp_vec,
            }
        }
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T> + Copy> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {

        if self.col != rhs.row{
            panic!();
        }
        let t_size: usize = self.row * rhs.col;
        let rows:usize = self.row;
        //let col_s:usize = t_size;
        let cols:usize = self.col;
        let mut temp_vec: Vec<T> = Vec::new();
        let mut cur_row:usize = 0;
        /*while cur_row < t_size{
            let mut r_cols:usize = cols * cur_row + cols;
            for l in cur_row..r_cols{
                let mut t: Vec<T> = Vec::new();
                for m in cur_row..r_cols{
                    t[0] = t[0] + self.data[m] * rhs.data[m * rows + l];
                }
                temp_vec.push(t[0]);
            }
            if cur_row == 0{
                cur_row += 1;
            }
            cur_row *= rows;
        }*/
        /*let b:usize = 0;
        for x in 0..self.row{
            let temp_c: usize = rows*b;
            for i in (rows * b)..(self.col + temp_c){
                let mut t: Vec<T> = Vec::new();
                for l in 0..rhs.row{
                    t[0] = t[0] + self.data[l] * rhs.data[l * rhs.col];
                }
            temp_vec.push(t[0]);
            }
            b += 1;
        }*/

        for i in 0..self.row{
            for j in 0..rhs.col{
                let mut t: Vec<T> =Vec::new();
                t.push(self.data[i] - self.data[i]);
                for k in 0..self.col{
                    t[0] = t[0] + self.data[i * self.col + k] * rhs.data[k * rhs.col + j];
                }
                temp_vec.push(t[0]);
            }
        }

        Matrix{
            row: self.row,
            col: rhs.col,
            data:temp_vec,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //unimplemented!();
        let mut f_str = String::new();
        let mut c:usize = 0;
        let cols:usize = self.col;
        let t_size: usize = self.row * self.col;
        let mut x:usize = 0;
        /*for l in 0..self.row{
            for i in (l*self.row)..(self.row*l+l){
                f_str = f_str + &self.data[i].to_string();
                if i == (self.col*l+l){
                    f_str.push('\n');
                }
                else{
                    f_str.push(' ');
                }
            }
        }*/
        for l in 0..t_size{
            f_str = f_str + &self.data[l].to_string();
            x += 1;
            if x == self.col{
                f_str.push('\n');
                x = 0;
            }
            else{
                f_str.push(' ');
            }

        }
        write!(f,"{}",f_str)

    }
}

#[cfg(test)] mod tests {

use Matrix;

#[test]fn it_works() { }

#[test]fn test_new() { let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]); println!("{:?}", x.row); println!("{:?}", x.col); println!("{:?}", x.data); assert_eq!(x.row, 2 as usize); assert_eq!(x.col, 3 as usize); assert_eq!(x.data[0], -2); }

#[test]fn test_new_empty() { let x: Matrix<i32> = Matrix::new_empty(2,3); assert_eq!(x.row, 2); assert_eq!(x.col, 3); }

#[test]fn test_data() { let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]); let shared_ref = x.data(); assert_eq!(x.data[0], shared_ref[0]); }

#[test]fn test_mut_data() { let mut x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]); let data = x.mut_data(); data[0] = 5; assert_eq!(data[0], 5); }

#[test]fn test_size() { let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]); let y = x.size(); assert_eq!(y.0, 2); assert_eq!(y.1, 3); }

//#[test]fn test_add() { let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]); let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]); let z = x + y; assert_eq!(z.data, [-1,0,1,2,3,4]); assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n"); }

#[test]fn test_sub() { let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]); let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]); let z = x - y; assert_eq!(z.data, [-3,-2,-1,0,1,2]); }

//#[test] #[should_panic] #[ignore] fn test_add_panic() { let x = Matrix::new(1, 3, &[-2, -1, 0, 1, 2, 3]); let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]); let _z = x + y; }
//#[test] #[should_panic] #[ignore] fn test_sub_panic() { let x = Matrix::new(1, 3, &[-2, -1, 0, 1, 2, 3]); let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]); let _z = x - y; }

#[test]fn test_fmt() { let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]); assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n"); }

#[test]fn test_mul() { let x = Matrix::new(3, 3, &[1,2,3,4,5,6,7,8,9]); let y = Matrix::new(3, 3, &[9,8,7,2,2,2,1,1,1]); let z = x * y; assert_eq!(z.data, [16,15,14,52,48,44,88,81,74]);}

}
