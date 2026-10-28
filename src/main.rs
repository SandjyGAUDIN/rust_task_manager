use std::io; 
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

fn sauvegarder_tâches(tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>>{
    let json = serde_json::to_string_pretty(tasks)?;
    std::fs::write("tasks.json", json)?;

    Ok(())
}

fn charger_tâches() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let chemin = std::path::Path::new("tasks.json");

    if !chemin.exists() {
        println!("Pas encore de fichier JSON, on démarre avec une liste vide");
        return Ok(Vec::new());
    }

    let contenu = std::fs::read_to_string(chemin)?;
    let tasks = serde_json::from_str(&contenu)?;

    Ok(tasks)
}

fn afficher_taches (tasks: &[Task]) {
    println!("\n--- Liste des taches ---");

    for task in tasks {
        println!(
            "#{} [{}] {}",
            task.id,
            if task.completed { "x" } else { " " },
            task.title
        );
    }
}

fn terminer_tache(tasks: &mut [Task], id: u32) {
    let resultat = tasks.iter_mut().find(|task|task.id == id);

    match resultat {
        Some(task) => {
            task.completed = true;
            println!("Tâche terminée !");
        }
        None => {
            println!("Aucune tâche avec cet ID.");
        }
    }
}

fn ajouter_tache(tasks: &mut Vec<Task>) {
    loop {
        let mut titre = String::new();

        println!("Entrez le titre de votre nouvelle tache");

        io::stdin()
            .read_line(&mut titre)
            .expect("Erreur lors de la lecture.");

        let titre = titre.trim().to_string();

        if titre.is_empty() {
            println!("Le titre ne peut pas être vide.");
            continue;
        }

        let id = tasks.len() as u32 + 1;

        let nouvelle_tache = Task {
            id,
            title: titre,
            completed: false,
        };

        tasks.push(nouvelle_tache);

        println!("Tâche ajoutée !");
        break;
    }
}

fn main() {

    let mut tasks = match charger_tâches() {
        Ok(tasks ) => tasks,
        Err(e) => {
            println!("Erreur lors du chargement des tâches : {e}");
            Vec::new()
        }
    };

    loop {
        println!("=== Gestionnaire des tâches ===");
        println!("1. Afficher les tâches");
        println!("2. Ajouter une tâche");
        println!("3. Marquer une tâche comme complété");
        println!("4. Quitter");
        println!();

        println!("Votre choix :");

        let mut choix = String::new();

        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur lors de la lecture");

        match choix.trim() {
            "1" => afficher_taches(&tasks),
            "2" => ajouter_tache(&mut tasks),
            "3" => {
                println!("Quel ID de tâche veux-tu terminer ?");

                let mut saisie = String::new();

                io::stdin()
                    .read_line(&mut saisie)
                    .expect("Erreur lors de la lecture");

                let id = match saisie.trim().parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Veuillez entrez un id numérique valide.");
                        continue;
                    }
                };

                terminer_tache(&mut tasks, id);
            }
            "4" => {
                match sauvegarder_tâches(&tasks) {
                    Ok(()) => println!("Tâches sauvegardées !"),
                    Err(e) => println!("Erreur lors de la sauvegarde : {e}"),
                }
                break;
            }
            _ => println!("Choix invalide"),
        }
    }
}