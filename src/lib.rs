enum Error {
    None = 0x00,
    InvalidRpc = 0x01,
    UnknownRpc = 0x02,
    UnableToConnect = 0x03,
    NotAuthorized = 0x04,
    Unknown = 0xff
}

enum State {
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

enum ImprovCommand {
    WifiSettings { ssid: String, password: String },
    GetCurrentState,
    GetDeviceInfo,
    GetWifiNetworks
}

impl ImprovCommand {


    fn from_bytes(data: &[u8]) -> Result<ImprovCommand, Error> {
        let cmd = CommandIdentifier::try_from(data[0])?;

        match cmd {
            CommandIdentifier::WifiSettings => {
                let ssid_length = data[2];
                let ssid_start = 3;
                let ssid_end = ssid_start + ssid_length;

                let pass_length = data[ssid_end];
                let pass_start = ssid_end + 1;
                let pass_end = pass_start + pass_length;

                let ssid = String::from_utf8(data[ssid_start..ssid_end]).map_err(|_| Error::Unknown)?;
                let password = String::from_utf8(data[pass_start..pass_end]).map_err(|_| Error::Unknown)?;

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
        assert_eq!(true, true);
    }
}
