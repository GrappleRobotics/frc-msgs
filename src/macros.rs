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
          _ => Err(GrappleError::FailedAssertion(alloc::borrow::Cow::Borrowed("Not Symmetric").into()))
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
            _ => Err(GrappleError::FailedAssertion(alloc::borrow::Cow::Borrowed("Received request, expected an ACK").into()))
          },
          _ => Err(GrappleError::FailedAssertion(alloc::borrow::Cow::Borrowed("Not Symmetric").into()))
        }
      )
    }
  }
}
