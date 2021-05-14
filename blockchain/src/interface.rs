use cursive::views::{Dialog, TextView, EditView, LinearLayout};
use cursive::{event::Key, menu, Cursive};
use cursive::traits::*;
use crate::blockchain::Blockchain;

pub fn start_interface() {
    let mut siv = cursive::default();

    let chain = Blockchain::new();

    siv.set_user_data(chain);

    siv.menubar()
        .add_subtree(
            "Actions",
            menu::MenuTree::new()
                .leaf("New Transaction", |s| { 
                    s.add_layer(new_transaction_popup())
                })
                .leaf("Current Transactions", |s| {
                    let chain = s.take_user_data::<Blockchain>().unwrap();
                    let dialog = Dialog::info(format!("{:?}", chain.current_transactions));

                    s.set_user_data(chain);
                    s.add_layer(dialog);
                })
                .leaf("Show Balances", |s| {
                    let chain = s.take_user_data::<Blockchain>().unwrap();
                    let dialog = Dialog::info(format!("{:?}", chain.balances()));

                    s.set_user_data(chain);
                    s.add_layer(dialog);
                })
                .leaf("Create Block", |s| {
                    let mut chain = s.take_user_data::<Blockchain>().unwrap();

                    let last_proof = chain.last_proof();
                    let new_proof = Blockchain::proof_of_work(last_proof);

                    // Hypothetical miner gets one coin for completing the new proof
                    chain.create_coin("minerA".to_string(), 1.);
                    chain.new_block(new_proof);

                    s.set_user_data(chain);
                    s.add_layer(Dialog::info("Creating block"));
                })
                .leaf("View Chain", |s| {
                    let chain = s.take_user_data::<Blockchain>().unwrap();
                    let dialog = Dialog::around(
                            TextView::new(format!("{}", chain))
                            .scrollable()
                        )
                        .button("Ok", |s| { s.pop_layer(); } );

                    s.set_user_data(chain);
                    s.add_layer(dialog);
                })
        )
        .add_leaf(
            "Help",
            |s| {
                s.add_layer(Dialog::info("Help screen"))
            }
        )
        .add_leaf(
            "Quit",
            |s| s.quit()
        );

    siv.set_autohide_menu(false);

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    // siv.add_global_callback(Key::Up, |s| s.select_menubar());

    //siv.add_layer(Dialog::around(TextView::new("Press <ESC> to access menu!"))
    //    .title("Local Blockchain"));

    siv.run();
}

fn new_transaction_popup() -> Dialog {
    let dialog = Dialog::new()
        .title("New Transaction")
        .content(
            LinearLayout::vertical()
                .child(TextView::new("Sender:"))
                .child(EditView::new().with_name("sender"))
                .child(TextView::new("Recipient:"))
                .child(EditView::new().with_name("recipient"))
                .child(TextView::new("Amount:"))
                .child(EditView::new().with_name("amount"))
        )
        .button(
            "CONFIRM",
            |s| { 
                let sender = s.call_on_name("sender", |view: &mut EditView| view.get_content()).unwrap();
                let recipient = s.call_on_name("recipient", |view: &mut EditView| view.get_content()).unwrap();
                let amount = s.call_on_name("amount", |view: &mut EditView| view.get_content()).unwrap();

                process_transaction(s, &sender, &recipient, &amount);
            }
        )
        .button(
            "Close",
            |s| { s.pop_layer(); }
        );

    dialog
}

// Extract blockchain, add transaction and store back in state
fn process_transaction(s: &mut Cursive, sender: &str, recipient: &str, amount: &str) {
    let amount: f64 = amount.parse().unwrap();

    let mut chain = s.take_user_data::<Blockchain>().unwrap();

    let balances = chain.balances();

    if !balances.contains_key(&sender.to_string()) {
        s.add_layer(Dialog::info("Sender must have balance in blockchain"));
        return
    }

    s.pop_layer();

    chain.new_transaction(sender.to_string(), recipient.to_string(), amount);
    s.add_layer(Dialog::info("New transaction added"));
    s.set_user_data(chain);
}
