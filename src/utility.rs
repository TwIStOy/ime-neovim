use rmpv::Value;
use std::collections::HashMap;
use std::convert::TryFrom;

pub fn into_dict(v: Value) -> Result<HashMap<String, Value>, Value> {
  let mut res: HashMap<String, Value> = HashMap::new();

  let mp = Vec::<(Value, Value)>::try_from(v)?;

  for (k, v) in mp {
    res.insert(String::try_from(k)?, v);
  }

  Ok(res)
}

