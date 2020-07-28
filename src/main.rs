extern crate serialport;

pub mod test;

fn main()
{
    print!("{0}", test::show_serialport_list());
}


