mod sales;
use sales::*;

fn main() {
    // -------- Stage 1 --------
    let mut vec_leads = Vec::new();

    add_to_vec(&mut vec_leads, Lead {
        name: "Alice".to_string(),
        contact: "alice@example.com".to_string(),
        potential_value: 12000.0,
    });

    println!("--- Stage 1: Viewing from Vec ---");
    view_from_vec(&vec_leads);

    // -------- Stage 2 & 3 --------
    let mut map_leads = std::collections::HashMap::new();

    add_lead(&mut map_leads, 1, Lead {
        name: "Bob".to_string(),
        contact: "bob@example.com".to_string(),
        potential_value: 15000.0,
    });

    println!("--- Stage 2: View Leads from HashMap ---");
    view_leads(&map_leads);

    // -------- Stage 3: Edit & Cancel --------
    if let Some(previous_lead) = edit_lead(&mut map_leads, 1, "Bobby".to_string()) {
        println!("--- After Edit ---");
        view_leads(&map_leads);

        // Cancel the edit
        cancel_edit(&mut map_leads, 1, previous_lead);

        println!("--- After Cancel Edit ---");
        view_leads(&map_leads);
    }
}
