use rui::*;

fn main() {
    rui(hstack! {
        circle()
            .color(RED_HIGHLIGHT)
            .tap(|| { println!("tapped circle") })
            .padding(Auto);
        rectangle(5.0)
            .color(AZURE_HIGHLIGHT)
            .drag(|offset| { println!("dragged on rectangle {:?}", offset) })
            .padding(Auto)
    });
}
