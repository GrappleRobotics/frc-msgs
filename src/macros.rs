#[macro_export]
macro_rules! symmetric {
  ($data:ident, $($tt:tt)*) => {
    {
      (
        |$data| $($tt)*,
        |msg| match msg {
          $($tt)* => {
            Ok($data)
          },
          _ => Err("Not Symmetric")
        }
      )
    }
  }
}

#[macro_export]
macro_rules! request_factory {
  ($data:ident, $($tt:tt)*) => {
    {
      (
        |$data| {
          let $data = Request::Request($data);
          $($tt)*
        },
        |msg| match msg {
          $($tt)* => match $data {
            Request::Ack(ack) => Ok(ack),
            _ => Err("Replied with Request")
          },
          _ => Err("Not Symmetric")
        }
      )
    }
  }
}

// #[cfg(test)]
// mod tests {
//     use crate::grapple::{GrappleDeviceMessage, Request, lasercan::LaserCanMessage};

//   #[test]
//   fn test() {
//     let (e, d) = request_factory!(data, GrappleDeviceMessage::DistanceSensor(LaserCanMessage::SetRange(data)));
    
//     let req = e(true);
//     let rsp: GrappleDeviceMessage = todo!();
//     let rsp = d(rsp);
//   }
// }