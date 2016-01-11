extern crate hello_rusty_worlds;

#[test]
fn called_from_outside_crate() {
  assert_eq!(hello_rusty_worlds::hello_world(0), Some("Hello Sol!"));
}
