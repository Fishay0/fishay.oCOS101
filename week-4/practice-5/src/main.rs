fn main() {
   let fullname = " Pan-atlantic University ";
   println!();
   println!("Name: {}", fullname);
   println!("Before trim ");
   println!("Length is {}",fullname.len());
   println!();
   println!("After trim ");
   println!("Length is {}",fullname.trim().len());
}
