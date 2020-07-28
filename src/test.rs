extern crate serialport;

pub fn show_serialport_list() -> usize
{
    if let Ok(ports) = serialport::available_ports()
    {
        match ports.len()
        {
            0 => println!("No ports found."),
            1 => println!("Found 1 port:"),
            n => println!("Found {} ports:", n),
        };

        let length = ports.len();
        
        for p in ports
        {
            println!("  {0} / {1:?}", p.port_name, p.port_type);
        }

        return length
    }
    else
    {
        print!("Error listing serial ports");
    }

    return 0
}


#[cfg(test)]
mod tests {
    use super::show_serialport_list;

    #[test]
    fn test_show_serialport_list() {
        assert_eq!(show_serialport_list(), 0)
    }
}


pub fn connect_serialport() -> usize
{
    if let Ok(ports) = serialport::available_ports()
    {
        match ports.len()
        {
            0 => println!("No ports found."),
            1 => println!("Found 1 port:"),
            n => println!("Found {} ports:", n),
        };

        let length = ports.len();
        
        for p in ports
        {
            println!("  {0} / {1:?}", p.port_name, p.port_type);
        }

        return length
    }
    else
    {
        print!("Error listing serial ports");
    }

    return 0
}
