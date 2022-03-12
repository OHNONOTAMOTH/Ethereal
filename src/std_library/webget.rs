use reqwest::blocking;

fn gethttp(i: String) -> Result<String, reqwest::Error> {
  let h = reqwest::blocking::get()?.text();
  return h;
}
