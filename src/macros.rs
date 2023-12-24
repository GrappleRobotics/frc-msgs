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
          _ => Err(GrappleError::FailedAssertion(CowStr::Borrowed("Not Symmetric")))
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
            _ => Err(GrappleError::FailedAssertion(CowStr::Borrowed("Received request, expected an ACK")))
          },
          _ => Err(GrappleError::FailedAssertion(CowStr::Borrowed("Not Symmetric")))
        }
      )
    }
  }
}
