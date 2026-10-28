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


fn charger_taches() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let contenu = match std::fs::read_to_string("tasks.json") {
        Ok(contenu) => contenu,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    return Ok(Vec::new());
                }
                _ => {
                    return Err(e.into());
                }
            }
        }
    };

    let tasks = serde_json::from_str(&contenu)?;

    Ok(tasks)
}



fn supprimer_tache(tasks: &mut Vec<Task>) {
    println!("Quelle tâche souhaitez-vous supprimer ?");

    let mut saisie = String::new();

    io::stdin()
        .read_line(&mut saisie)
        .expect("Erreur lors de la lecture");

    let id = match saisie.trim().parse::<u32>() {
        Ok(id) => id,
        Err(_) => {
            println!("Veuillez entrer un ID numérique valide.");
            return;
        }
    };

    match tasks.iter().position(|task| task.id == id) {
        Some(index) => {
            tasks.remove(index);
            println!("Tâche supprimée !");
            match sauvegarder_tâches(&tasks) {
                Ok(()) => println!("Tâches sauvegardées !"),
                Err(e) => println!("Erreur lors de la sauvegarde : {e}"),
            } 
        }
        None => {
            println!("Aucune tâche trouvée avec cet ID.");
        }
    }
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

fn modifier_tache(tasks: &mut [Task]) {
    let mut id = String::new();

    println!("Quelle tâche souhaitez-vous modifier ? Entrez son ID :");

    io::stdin()
        .read_line(&mut id)
        .expect("Erreur lors de la lecture");

    let id = match id.trim().parse::<u32>() {
        Ok(id) => id,
        Err(_) => {
            println!("Veuillez entrer un ID numérique valide.");
            return;
        }
    };

    let resultat = tasks.iter_mut().find(|tasks| tasks.id == id);

    match resultat {
        Some(task) => {
            let mut nouveau_titre = String::new();

            println!("Qu'elle est le titre que vous souhaitez mettre à la tâche : {id}");

            io::stdin()
                .read_line(&mut nouveau_titre)
                .expect("Erreur lors de la lecture");

            let nouveau_titre = nouveau_titre.trim().to_string();

            if nouveau_titre.is_empty() {
                println!("Le titre ne peut pas être vide.");
                return;
            } else {
                task.title = nouveau_titre;
                println!("Titre de la tâche modifié !");
                match sauvegarder_tâches(&tasks) {
                    Ok(()) => println!("Tâches sauvegardées !"),
                    Err(e) => println!("Erreur lors de la sauvegarde : {e}"),
                } 
            }
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

        let max_id = tasks.iter().map(|task| task.id).max().unwrap_or(0);

        let id = match max_id.checked_add(1) {
            Some(id) => id,
            None => {
                println!("Impossible de créer une tâche : plus aucun ID disponible.");
                return;
            }
        };

        let nouvelle_tache = Task {
            id,
            title: titre,
            completed: false,
        };

        tasks.push(nouvelle_tache);

        println!("Tâche ajoutée !");
        break;
    };
}

fn main() {

    let mut tasks = match charger_taches() {
        Ok(tasks ) => tasks,
        Err(e) => {
            println!("Erreur lors du chargement des tâches : {e}");
            return;
        }
    };

    loop {
        println!("=== Gestionnaire des tâches ===");
        println!("1. Afficher les tâches");
        println!("2. Ajouter une tâche");
        println!("3. Marquer une tâche comme complété");
        println!("4. Marquer une tâche comme complété");
        println!("5. Modifier le titre d'une tâche");
        println!("6. Quitter");
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
                supprimer_tache(&mut tasks);
            }
            "5" => modifier_tache(&mut tasks),
            "6" => {
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