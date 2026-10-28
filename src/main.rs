struct Task {
    id: u32,
    title: String,
    completed: bool,
}

use std::io; 

fn main() {
    let mut tasks: Vec<Task> = vec![
        Task {
            id: 1,
            title: String::from("Apprendre Rust"),
            completed: false,
        },
        Task {
            id: 2,
            title: String::from("Créer le menu tâche"),
            completed: true,
        },
    ];

    loop {
        println!("=== Gestionnaire des tâches ===");
        println!("1. Afficher les tâches");
        println!("2. Ajouter une tâche");
        println!("3. Quitter");
        println!();

        println!("Votre choix :");

        let mut choix = String::new();

        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur lors de la lecture");

        match choix.trim() {
            "1" => {
                println!("\n--- liste des tâches ---");
                
                for task in &tasks {
                    println!(
                        "# {} [{}] {}",
                        task.id,
                        if task.completed {"x"} else { " " },
                        task.title
                    );
                }
            },
            "2" => println!("Ajout d'une tache..."),
            "3" => {
                println!("Aurevoir !");
                break;
            }
            _ => println!("Choix invalide"),
        }
    }
}