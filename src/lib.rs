extern crate serialport;

pub fn test()
{
    if let Ok(ports) = serialport::available_ports()
    {
        match ports.len()
        {
            0 => println!("No ports found."),
            1 => println!("Found 1 port:"),
            n => println!("Found {} ports:", n),
        };
        
        for p in ports
        {
            println!("  {0} / {1:?}", p.port_name, p.port_type);
        }
    }
    else
    {
        print!("Error listing serial ports");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
