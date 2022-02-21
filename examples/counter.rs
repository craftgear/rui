use rui::*;

fn main() {
    rui(state(1, |count: State<usize>| {
        vstack! {
            text(&format!("{:?}", *count.get()))
                .padding(Auto);
            button("increment", move || {
                *count.get() += 1;
            })
                .padding(Auto)
        }
    }));
}
