//struct Notice is used to normalise the message format we send with ZeroMQ
use zmq::Socket;


pub enum NT {   //notice type
    P,              //for position msg
    S,              //for speed msg
    C,              //for callsign msg
    D,              //for data_base msg
    N,              //for Null msg
}

pub struct Notice {
    pub nt: NT,
    pub icao: String,
    pub data: String,
}

impl Notice {
    pub fn into_string(&self) -> String {
        let mut res = self.icao.clone();
        res.push_str("|");
        match self.nt {
            NT::P => res.push_str("P|"),
            NT::S => res.push_str("S|"),
            NT::C => res.push_str("C|"),
            NT::D => res.push_str("D|"),
            NT::N => res.push_str("N|"),
        };
        res.push_str(&self.data.clone());
        return res;
    }

    pub fn send(&self, sock: &Socket)-> () {
        sock.send(&self.into_string(), 0).unwrap();

    }
}