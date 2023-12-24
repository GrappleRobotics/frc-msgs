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
          _ => Err(GrappleError::FailedAssertion("Not Symmetric".to_string()))
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
          _ => Err(GrappleError::FailedAssertion("Not Symmetric".to_string()))
        }
      )
    }
  }
}
