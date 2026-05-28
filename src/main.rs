use std::process::Command;
use std::time::Duration;
use std::thread;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 1 && args[1] == "--help" {
        println!("Usage: pomodoro [options]");
        println!("Options:");
        println!("  --help       Affiche ce message d'aide");
        println!("  --version    Affiche la version du programme");
        return;
    } else if args.len() > 1 && args[1] == "--version" {
        println!("Pomodoro Timer version 1.0");
        return;
    } else if args.len() > 1 {
        println!("Option inconnue : {}", args[1]);
        println!("Utilisez --help pour voir les options disponibles.");
        return;
    }
    let bk_dur = 5 * 60;
    let pd_dur = 25 * 60;
    let mut it = 4;
    println!("Démarrage du Pomodoro Timer : {:02}:{:02}", pd_dur / 60, pd_dur % 60);
    while it > 0 {
        for i in (0..=pd_dur).rev() {
            println!("Temps restant avant la pause : {:02}:{:02}", i / 60, i % 60);
            thread::sleep(Duration::from_secs(1));
        }
        Command::new("notify-send")
            .arg("Pomodoro Timer")
            .arg("Timer terminé ! Prenez une pause de 5 minutes.")
            .output()
            .expect("Failed to send notification");
        for i in (0..=bk_dur).rev() {
            println!("Temps de pause restant : {:02}:{:02}", i / 60, i % 60);
            thread::sleep(Duration::from_secs(1));
        }
        Command::new("notify-send")
            .arg("Pomodoro Timer")
            .arg("Pause terminée ! Reprenez le travail.")
            .output()
            .expect("Failed to send notification");
        it -= 1;
    }
}
