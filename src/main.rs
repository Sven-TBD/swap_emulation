mod swap;
fn main() {
    println!("Hello, world!");
    let mut usdc_pool =  swap::Pool::new(5000, 10000000);
    usdc_pool.user_buy_token(2);

}

// fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
//     // Put the buttons and label on the same row:
//     ui.horizontal(|ui| {
//         if ui.button("-").clicked() {
//             *counter -= 1;
//         }
//         ui.label(counter.to_string());
//         if ui.button("+").clicked() {
//             *counter += 1;
//         }
//     });
// }