use crate::protocol::{*};


pub fn check(header: &Header, vec_data: &Vec<u8>) -> Data
{
    if header.length != vec_data.len() as u8 {
        return Data::ErrorMessage(format!("Handler / Data Length Error / DataType: {:?}, Length: {}, Data: {:?}", header.data_type, header.length, vec_data));
    }

    let length : usize = header.length as usize;

    match header.data_type {
        // Ping = 0x01
        DataType::Ping => {
            match Ping::parse(vec_data) {
                Ok(data) => return Data::Ping(data),
                Err(_e) => {},
            }
        },
        // Ack = 0x02
        DataType::Ack => {
            match Ack::parse(vec_data) {
                Ok(data) => return Data::Ack(data),
                Err(_e) => {},
            }
        },
        // Error = 0x03
        DataType::Error => {
            match Error::parse(vec_data) {
                Ok(data) => return Data::Error(data),
                Err(_e) => {},
            }
        },
        // Request = 0x04
        DataType::Request => {
            if length == Request::size() {
                match Request::parse(vec_data) {
                    Ok(data) => return Data::Request(data),
                    Err(_e) => {},
                }
            }
            else if length == RequestOption::size() {
                match RequestOption::parse(vec_data) {
                    Ok(data) => return Data::RequestOption(data),
                    Err(_e) => {},
                }
            }
        },
        // Address = 0x06
        DataType::Address => {
            match Address::parse(vec_data) {
                Ok(data) => return Data::Address(data),
                Err(_e) => {},
            }
        },
        // Information = 0x07
        DataType::Information => {
            match Information::parse(vec_data) {
                Ok(data) => return Data::Information(data),
                Err(_e) => {},
            }
        },
        // Update = 0x08
        DataType::Update => {
            match Update::parse(vec_data) {
                Ok(data) => return Data::Update(data),
                Err(_e) => {},
            }
        },
        // UpdateLocation = 0x09
        DataType::UpdateLocation => {
            match UpdateLocation::parse(vec_data) {
                Ok(data) => return Data::UpdateLocation(data),
                Err(_e) => {},
            }
        },
        // SystemInformation = 0x0C
        DataType::SystemInformation => {
            match SystemInformation::parse(vec_data) {
                Ok(data) => return Data::SystemInformation(data),
                Err(_e) => {},
            }
        },
        // Administrator = 0x0E
        DataType::Administrator => {
            match Administrator::parse(vec_data) {
                Ok(data) => return Data::Administrator(data),
                Err(_e) => {},
            }
        },

        // Monitor = 0x0F
        DataType::Monitor => {
            let monitor_header_type = monitor::HeaderType::from_u8(vec_data[0]);
            match monitor_header_type {
                monitor::HeaderType::Monitor0 => {
                    match monitor::Monitor0::parse(&vec_data[1..]) {
                        Ok(data) => return Data::Monitor0(data),
                        Err(_e) => {},
                    }
                },
                monitor::HeaderType::Monitor4 => {
                    match monitor::Monitor4::parse(&vec_data[1..]) {
                        Ok(data) => return Data::Monitor4(data),
                        Err(_e) => {},
                    }
                },
                monitor::HeaderType::Monitor8 => {
                    match monitor::Monitor8::parse(&vec_data[1..]) {
                        Ok(data) => return Data::Monitor8(data),
                        Err(_e) => {},
                    }
                },
            }
        },

        // Control = 0x10
        DataType::Control => {
            if length == control::Quad8::size() {
                match control::Quad8::parse(vec_data) {
                    Ok(data) => return Data::Quad8(data),
                    Err(_e) => {},
                }
            }
            else if length == control::Quad8AndRequestData::size() {
                match control::Quad8AndRequestData::parse(vec_data) {
                    Ok(data) => return Data::Quad8AndRequestData(data),
                    Err(_e) => {},
                }
            }
            else if length == control::Position16::size() {
                match control::Position16::parse(vec_data) {
                    Ok(data) => return Data::ControlPosition16(data),
                    Err(_e) => {},
                }
            }
            else if length == control::Position::size() {
                match control::Position::parse(vec_data) {
                    Ok(data) => return Data::ControlPosition(data),
                    Err(_e) => {},
                }
            }
        },

        // Control = 0x11
        DataType::Command => {
            if length == command::Command::size() {
                match command::Command::parse(vec_data) {
                    Ok(data) => return Data::Command(data),
                    Err(_e) => {},
                }
            }
            else if length == command::CommandLightEvent::size() {
                match command::CommandLightEvent::parse(vec_data) {
                    Ok(data) => return Data::CommandLightEvent(data),
                    Err(_e) => {},
                }
            }
            else if length == command::CommandLightEventColor::size() {
                match command::CommandLightEventColor::parse(vec_data) {
                    Ok(data) => return Data::CommandLightEventColor(data),
                    Err(_e) => {},
                }
            }
        },

        // Pairing = 0x12
        DataType::Pairing => {
            match communication::Pairing::parse(vec_data) {
                Ok(data) => return Data::Pairing(data),
                Err(_e) => {},
            }
        },
        // Rssi = 0x13
        DataType::Rssi => {
            match communication::Rssi::parse(vec_data) {
                Ok(data) => return Data::Rssi(data),
                Err(_e) => {},
            }
        },

        // LightManual = 0x20
        DataType::LightManual => {
            match light::Manual::parse(vec_data) {
                Ok(data) => return Data::Manual(data),
                Err(_e) => {},
            }
        },
        // LightMode = 0x21
        DataType::LightMode => {
            match light::Mode::parse(vec_data) {
                Ok(data) => return Data::LightMode(data),
                Err(_e) => {},
            }
        },
        // LightEvent = 0x22
        DataType::LightEvent => {
            match light::Event::parse(vec_data) {
                Ok(data) => return Data::LightEvent(data),
                Err(_e) => {},
            }
        },

        // RawMotion = 0x30
        DataType::RawMotion => {
            match sensor::RawMotion::parse(vec_data) {
                Ok(data) => return Data::RawMotion(data),
                Err(_e) => {},
            }
        },
        // RawFlow = 0x31
        DataType::RawFlow => {
            match sensor::RawFlow::parse(vec_data) {
                Ok(data) => return Data::RawFlow(data),
                Err(_e) => {},
            }
        },

        // Attitude = 0x41
        DataType::Attitude => {
            match sensor::Attitude::parse(vec_data) {
                Ok(data) => return Data::Attitude(data),
                Err(_e) => {},
            }
        },
        // Position = 0x42
        DataType::Position => {
            if length == sensor::Position::size() {
                match sensor::Position::parse(vec_data) {
                    Ok(data) => return Data::Position(data),
                    Err(_e) => {},
                }
            }
            else if length == sensor::PositionVelocity::size() {
                match sensor::PositionVelocity::parse(vec_data) {
                    Ok(data) => return Data::PositionVelocity(data),
                    Err(_e) => {},
                }
            }
        },
        // Motion = 0x44
        DataType::Motion => {
            match sensor::Motion::parse(vec_data) {
                Ok(data) => return Data::Motion(data),
                Err(_e) => {},
            }
        },
        // Range = 0x45
        DataType::Range => {
            match sensor::Range::parse(vec_data) {
                Ok(data) => return Data::Range(data),
                Err(_e) => {},
            }
        },

        // Count = 0x50
        DataType::Count => {
            match Count::parse(vec_data) {
                Ok(data) => return Data::Count(data),
                Err(_e) => {},
            }
        },
        // Bias = 0x51
        DataType::Bias => {
            match sensor::Bias::parse(vec_data) {
                Ok(data) => return Data::Bias(data),
                Err(_e) => {},
            }
        },
        // Trim = 0x52
        DataType::Trim => {
            match sensor::Trim::parse(vec_data) {
                Ok(data) => return Data::Trim(data),
                Err(_e) => {},
            }
        },
        // LostConnection = 0x54
        DataType::LostConnection => {
            match communication::LostConnection::parse(vec_data) {
                Ok(data) => return Data::LostConnection(data),
                Err(_e) => {},
            }
        },
        // MagnetometerOffset = 0x55
        DataType::MagnetometerOffset => {
            match sensor::MagnetometerOffset::parse(vec_data) {
                Ok(data) => return Data::MagnetometerOffset(data),
                Err(_e) => {},
            }
        },

        // Motor = 0x60
        DataType::Motor => {
            if length == motor::MotorV::size() {
                match motor::MotorV::parse(vec_data) {
                    Ok(data) => return Data::MotorV(data),
                    Err(_e) => {},
                }
            }
            else if length == motor::MotorRV::size() {
                match motor::MotorRV::parse(vec_data) {
                    Ok(data) => return Data::MotorRV(data),
                    Err(_e) => {},
                }
            }
            else if length == motor::MotorVA::size() {
                match motor::MotorVA::parse(vec_data) {
                    Ok(data) => return Data::MotorVA(data),
                    Err(_e) => {},
                }
            }
            else if length == motor::MotorRVA::size() {
                match motor::MotorRVA::parse(vec_data) {
                    Ok(data) => return Data::MotorRVA(data),
                    Err(_e) => {},
                }
            }
        },

        // Motor = 0x61
        DataType::MotorSingle => {
            if length == motor::MotorSingleV::size() {
                match motor::MotorSingleV::parse(vec_data) {
                    Ok(data) => return Data::MotorSingleV(data),
                    Err(_e) => {},
                }
            }
            else if length == motor::MotorSingleRV::size() {
                match motor::MotorSingleRV::parse(vec_data) {
                    Ok(data) => return Data::MotorSingleRV(data),
                    Err(_e) => {},
                }
            }
        },

        // Buzzer = 0x62
        DataType::Buzzer => {
            if length == buzzer::Melody::size() {
                match buzzer::Melody::parse(vec_data) {
                    Ok(data) => return Data::Melody(data),
                    Err(_e) => {},
                }
            }
            else if length == buzzer::BuzzerScale::size() {
                match buzzer::BuzzerScale::parse(vec_data) {
                    Ok(data) => return Data::BuzzerScale(data),
                    Err(_e) => {},
                }
            }
            else if length == buzzer::BuzzerHz::size() {
                match buzzer::BuzzerHz::parse(vec_data) {
                    Ok(data) => return Data::BuzzerHz(data),
                    Err(_e) => {},
                }
            }
        },

        // Button = 0x70
        DataType::Button => {
            match button::Button::parse(vec_data) {
                Ok(data) => return Data::Button(data),
                Err(_e) => {},
            }
        },
        // Joystick = 0x71
        DataType::Joystick => {
            match joystick::Joystick::parse(vec_data) {
                Ok(data) => return Data::Joystick(data),
                Err(_e) => {},
            }
        },

        // DisplayClear = 0x80
        DataType::DisplayClear => {
            if length == display::ClearAll::size() {
                match display::ClearAll::parse(vec_data) {
                    Ok(data) => return Data::DisplayClearAll(data),
                    Err(_e) => {},
                }
            }
            else if length == display::Clear::size() {
                match display::Clear::parse(vec_data) {
                    Ok(data) => return Data::DisplayClear(data),
                    Err(_e) => {},
                }
            }
        },
        // Invert = 0x81
        DataType::DisplayInvert => {
            match display::Invert::parse(vec_data) {
                Ok(data) => return Data::DisplayInvert(data),
                Err(_e) => {},
            }
        },
        // DrawPoint = 0x82
        DataType::DisplayDrawPoint => {
            match display::DrawPoint::parse(vec_data) {
                Ok(data) => return Data::DisplayDrawPoint(data),
                Err(_e) => {},
            }
        },
        // DisplayDrawLine = 0x83
        DataType::DisplayDrawLine => {
            match display::DrawLine::parse(vec_data) {
                Ok(data) => return Data::DisplayDrawLine(data),
                Err(_e) => {},
            }
        },
        // DisplayDrawRect = 0x84
        DataType::DisplayDrawRect => {
            match display::DrawRect::parse(vec_data) {
                Ok(data) => return Data::DisplayDrawRect(data),
                Err(_e) => {},
            }
        },
        // DisplayDrawCircle = 0x85
        DataType::DisplayDrawCircle => {
            match display::DrawCircle::parse(vec_data) {
                Ok(data) => return Data::DisplayDrawCircle(data),
                Err(_e) => {},
            }
        },
        // DisplayDrawString = 0x86
        DataType::DisplayDrawString => {
            match display::DrawString::parse(vec_data) {
                Ok(data) => return Data::DisplayDrawString(data),
                Err(_e) => {},
            }
        },
        // DisplayDrawStringAlign = 0x87
        DataType::DisplayDrawStringAlign => {
            match display::DrawStringAlign::parse(vec_data) {
                Ok(data) => return Data::DisplayDrawStringAlign(data),
                Err(_e) => {},
            }
        },
        // DisplayDrawImage = 0x88
        DataType::DisplayDrawImage => {
            match display::DrawImage::parse(vec_data) {
                Ok(data) => return Data::DisplayDrawImage(data),
                Err(_e) => {},
            }
        },


        // NavigationTarget = 0xD0
        DataType::NavigationTarget => {
            if length == navigation::TargetMove::size() {
                match navigation::TargetMove::parse(vec_data) {
                    Ok(data) => return Data::NavigationTargetMove(data),
                    Err(_e) => {},
                }
            }
            else if length == navigation::TargetAction::size() {
                match navigation::TargetAction::parse(vec_data) {
                    Ok(data) => return Data::NavigationTargetAction(data),
                    Err(_e) => {},
                }
            }
        },
        // NavigationLocation = 0xD1
        DataType::NavigationLocation => {
            match navigation::Location::parse(vec_data) {
                Ok(data) => return Data::NavigationLocation(data),
                Err(_e) => {},
            }
        },
        // NavigationMonitor = 0xD2
        DataType::NavigationMonitor => {
            match navigation::Monitor::parse(vec_data) {
                Ok(data) => return Data::NavigationMonitor(data),
                Err(_e) => {},
            }
        },
        // NavigationHeading = 0xD3
        DataType::NavigationHeading => {
            match navigation::Heading::parse(vec_data) {
                Ok(data) => return Data::NavigationHeading(data),
                Err(_e) => {},
            }
        },
        // NavigationCounter = 0xD4
        DataType::NavigationCounter => {
            match navigation::Counter::parse(vec_data) {
                Ok(data) => return Data::NavigationCounter(data),
                Err(_e) => {},
            }
        },
        // NavigationSatellite = 0xD5
        DataType::NavigationSatellite => {
            match navigation::Satellite::parse(vec_data) {
                Ok(data) => return Data::NavigationSatellite(data),
                Err(_e) => {},
            }
        },
        // NavigationLocationAdjust = 0xD6
        DataType::NavigationLocationAdjust => {
            match navigation::LocationAdjust::parse(vec_data) {
                Ok(data) => return Data::NavigationLocationAdjust(data),
                Err(_e) => {},
            }
        },

        _ => {},
    }


    Data::ErrorMessage(format!("Handler / Parsing Error / DataType: {:?}, Length: {}, Data: {:?}", header.data_type, header.length, vec_data))
}

