use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box, Orientation};
use rand::Rng;
use std::sync::{Arc, Mutex};

fn main() {
    let app = Application::builder()
        .application_id("com.waddle.pomodoro")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Waddle")
            .default_width(400)
            .default_height(150)
            .build();

        let vbox = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .margin_top(20)
            .margin_bottom(20)
            .margin_start(20)
            .margin_end(20)
            .build();

        let sent = [
            "Vous allez y arriver !",
            "Votre volonté est plus forte que tout !",
            "Chaque minute compte, continuez !",
            "Vous êtes plus fort que la procrastination !",
            "Le succès est à portée de main, ne lâchez rien !",
        ];
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..sent.len());

        let title: Label = Label::builder()
            .label(sent[idx])
            .build();

        let timer: Label = Label::builder()
            .label("25:00")
            .build();

        let button = Button::with_label("Start");

        // Arc<Mutex<T>> permet de partager un état mutable entre plusieurs closures
        let seconds_left = Arc::new(Mutex::new(25 * 60u32));
        let timer_clone = timer.clone();
        let seconds_clone = Arc::clone(&seconds_left);

        button.connect_clicked(move |_| {
            let timer_ref = timer_clone.clone();
            let secs_ref = Arc::clone(&seconds_clone);

            glib::timeout_add_seconds_local(1, move || {
                let mut secs = secs_ref.lock().unwrap();
                if *secs == 0 {
                    timer_ref.set_label("Prenez une pause de 5 minutes !");
                    return glib::ControlFlow::Break;
                }
                *secs -= 1;
                timer_ref.set_label(&format!("{:02}:{:02}", *secs / 60, *secs % 60));
                glib::ControlFlow::Continue
            });
        });

        vbox.append(&title);
        vbox.append(&timer);
        vbox.append(&button);

        window.set_child(Some(&vbox));
        window.show();
    });

    app.run();
}

// use std::process::Command;
// use std::time::Duration;
// use std::thread;
// use std::env::args;

// fn main() {
//     let args: Vec<String> = args().collect();
//     if args.len() > 1 && args[1] == "--help" {
//         println!("Usage: pomodoro [options]");
//         println!("Options:");
//         println!("  --help       Affiche ce message d'aide");
//         println!("  --version    Affiche la version du programme");
//         return;
//     } else if args.len() > 1 && args[1] == "--version" {
//         println!("Pomodoro Timer version 1.0");
//         return;
//     } else if args.len() > 1 {
//         println!("Option inconnue : {}", args[1]);
//         println!("Utilisez --help pour voir les options disponibles.");
//         return;
//     }
//     let bk_dur = 5 * 60;
//     let pd_dur = 25 * 60;
//     let mut it = 4;
//     println!("Démarrage du Pomodoro Timer : {:02}:{:02}", pd_dur / 60, pd_dur % 60);
//     while it > 0 {
//         for i in (0..=pd_dur).rev() {
//             println!("Temps restant avant la pause : {:02}:{:02}", i / 60, i % 60);
//             thread::sleep(Duration::from_secs(1));
//         }
//         Command::new("notify-send")
//             .arg("Pomodoro Timer")
//             .arg("Timer terminé ! Prenez une pause de 5 minutes.")
//             .output()
//             .expect("Failed to send notification");
//         for i in (0..=bk_dur).rev() {
//             println!("Temps de pause restant : {:02}:{:02}", i / 60, i % 60);
//             thread::sleep(Duration::from_secs(1));
//         }
//         Command::new("notify-send")
//             .arg("Pomodoro Timer")
//             .arg("Pause terminée ! Reprenez le travail.")
//             .output()
//             .expect("Failed to send notification");
//         it -= 1;
//     }
// }