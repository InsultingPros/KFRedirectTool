pub const HTML_TEMPLATE1: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <title>File Download Server</title>
    <link rel="icon" href="/favicon.ico" type="image/x-icon">
    <link
      rel="stylesheet"
      href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css"
    >
  </head>
  <body>
    <h1>Available Files</h1>
    <ul>
      <li><a href="/download/doesnt_exist.txt">doesnt_exist.txt</a></li>
      <li><a href="/download/KF_Invasion.u.uz2">1 KF_Invasion.u.uz2</a></li>
      <li><a href="/KF_Invasion.u.uz2">2 KF_Invasion.u.uz2</a></li>
      <li><a href="KF_Invasion.u.uz2">3 KF_Invasion.u.uz2</a></li>
    </ul>
  </body>
</html>
"#;

#[allow(dead_code)]
pub const HTML_TEMPLATE2: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="color-scheme" content="light dark">
    <link rel="stylesheet" href="css/pico.min.css">
    <title>Hello world!</title>
  </head>
  <body>
    <main class="container">
      <h1>Hello world!</h1>
    </main>
  </body>
</html>
"#;
