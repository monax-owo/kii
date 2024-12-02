use bevy::{
  app::App,
  color::{palettes::css, Color},
  prelude::ClearColor,
  DefaultPlugins,
};
use hidapi::HidApi;

fn main() {
  // let api = HidApi::new().unwrap();
  // for device in api.device_list() {
  //   println!(
  //     "product: {:#?}, manufacturer: {:#?}",
  //     device.product_string().unwrap_or("None".into()),
  //     device.manufacturer_string().unwrap_or("None".into())
  //   );
  // }
  App::new()
    .insert_resource(ClearColor(Color::Srgba(css::LIGHT_CYAN)))
    .add_plugins(DefaultPlugins)
    .run();
}
