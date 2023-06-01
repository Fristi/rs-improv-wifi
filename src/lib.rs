use heapless::String;

#[derive(Debug, PartialEq)]
pub enum Error {
    None = 0x00,
    InvalidRpc = 0x01,
    UnknownRpc = 0x02,
    UnableToConnect = 0x03,
    NotAuthorized = 0x04,
    Unknown = 0xff
}

pub enum State {
    Stopped = 0x00,
    AwaitingAuthorization = 0x01,
    Authorized = 0x02,
    Provisioning = 0x03,
    Provisioned = 0x04
}

enum CommandIdentifier {
    WifiSettings = 0x01,
    GetCurrentState = 0x02,
    GetDeviceInfo = 0x03,
    GetWifiNetworks = 0x04
}

impl TryFrom<u8> for CommandIdentifier {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(CommandIdentifier::WifiSettings),
            0x02 => Ok(CommandIdentifier::GetCurrentState),
            0x03 => Ok(CommandIdentifier::GetDeviceInfo),
            0x04 => Ok(CommandIdentifier::GetWifiNetworks),
            _ => Err(Error::UnknownRpc)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ImprovCommand {
    WifiSettings { ssid: String<255>, password: String<255> },
    GetCurrentState,
    GetDeviceInfo,
    GetWifiNetworks
}

impl ImprovCommand {


    pub fn from_bytes(data: &[u8]) -> Result<ImprovCommand, Error> {
        let cmd = CommandIdentifier::try_from(data[0])?;

        match cmd {
            CommandIdentifier::WifiSettings => {
                let ssid_length = data[2] as usize;
                let ssid_start = 3;
                let ssid_end= ssid_start + ssid_length;

                let pass_length = data[ssid_end] as usize;
                let pass_start = ssid_end + 1;
                let pass_end = pass_start + pass_length;

                let ssid = core::str::from_utf8(&data[ssid_start..ssid_end]).map(|x| String::from(x)).map_err(|_| Error::Unknown)?;
                let password = core::str::from_utf8(&data[pass_start..pass_end]).map(|x| String::from(x)).map_err(|_| Error::Unknown)?;

                Ok(ImprovCommand::WifiSettings { ssid, password })
            },
            CommandIdentifier::GetDeviceInfo => Ok(ImprovCommand::GetDeviceInfo),
            CommandIdentifier::GetCurrentState => Ok(ImprovCommand::GetCurrentState),
            CommandIdentifier::GetWifiNetworks => Ok(ImprovCommand::GetWifiNetworks),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {


        let bytes: [u8; 32] = [0x01, 0x1e, 0x0c, 0x4d,0x79,0x57,0x69,0x72,0x65,0x6c,0x65,0x73,0x73,0x41,0x50, 0x10, 0x6d,0x79,0x73,0x65,0x63,0x75,0x72,0x65,0x70,0x61,0x73,0x73,0x77,0x6f,0x72,0x64];

        assert_eq!(ImprovCommand::from_bytes(&bytes), Ok(ImprovCommand::WifiSettings { ssid: String::from("MyWirelessAP"), password: String::from("mysecurepassword")}))
    }
}
