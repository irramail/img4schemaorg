use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::{ServerBuilder};

use img4schemaorg::*;

fn main() {
  let mut io = IoHandler::new();

  let _ = set_first_run();

  io.add_method("set_img",  move |params: Params| {
    let w = parse_arguments(params)?;
    let _ = fetch_img( &w[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("set",  move |params: Params| {
    let w = parse_arguments(params)?;
    let _ = all_settings( &w[0]);

    Ok(Value::String("".to_string()))
  });

  io.add_method("exists_img",  | _params | {
    let bool = exists_img().unwrap().to_string();
    Ok(Value::String(bool))
  });

  io.add_method("reTry",  | _params | {
    let _ = retry();
    Ok(Value::String("".to_string()))
  });

    io.add_method("upload",  | _params | {
        let _ = upload();
        Ok(Value::String("".to_string()))
    });

  io.add_method("get_schema_org",  | _params | {
    let schema = get_schema_org().unwrap();
    Ok(Value::String(schema))
  });

  let server = ServerBuilder::new(io)
  .threads(3)
  .start_http(&"127.0.0.1:3033".parse().unwrap())
  .unwrap();

  server.wait();
}
