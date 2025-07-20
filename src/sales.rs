use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Lead {
    pub name: String,
    pub contact: String,
    pub potential_value: f32,
}



pub fn add_to_vec(leads: &mut Vec<Lead>, lead: Lead) {
    leads.push(lead);
}

pub fn view_from_vec(leads: &Vec<Lead>) {
    for (i, lead) in leads.iter().enumerate() {
        println!(
            "Index: {}, Name: {}, Contact: {}, Value: ${}",
            i, lead.name, lead.contact, lead.potential_value
        );
    }
}



pub fn add_lead(leads: &mut HashMap<u32, Lead>, id: u32, lead: Lead) {
    leads.insert(id, lead);
}

pub fn view_leads(leads: &HashMap<u32, Lead>) {
    for (id, lead) in leads {
        println!(
            "ID: {}, Name: {}, Contact: {}, Value: ${}",
            id, lead.name, lead.contact, lead.potential_value
        );
    }
}

pub fn remove_lead(leads: &mut HashMap<u32, Lead>, id: u32) {
    if leads.remove(&id).is_none() {
        println!("Lead with ID {} not found", id);
    }
}

pub fn edit_lead(leads: &mut HashMap<u32, Lead>, id: u32, new_name: String) -> Option<Lead> {
    if let Some(lead) = leads.get_mut(&id) {
        let old = lead.clone(); 
        lead.name = new_name;
        Some(old) 
    } else {
        println!("Lead not found.");
        None
    }
}


pub fn cancel_edit(leads: &mut HashMap<u32, Lead>, id: u32, original: Lead) {
    leads.insert(id, original);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_add_and_view_vec() {
        let mut vec = Vec::new();
        add_to_vec(&mut vec, Lead {
            name: "Alice".into(),
            contact: "alice@example.com".into(),
            potential_value: 10000.0,
        });

        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0].name, "Alice");
    }

    #[test]
    fn test_add_and_view_hashmap() {
        let mut leads = HashMap::new();

        add_lead(&mut leads, 1, Lead {
            name: "Dave".into(),
            contact: "dave@example.com".into(),
            potential_value: 5000.0,
        });

        assert!(leads.contains_key(&1));
        assert_eq!(leads.get(&1).unwrap().name, "Dave");
    }

    #[test]
    fn test_remove_lead() {
        let mut leads = HashMap::new();

        add_lead(&mut leads, 1, Lead {
            name: "Derrickremove".into(),
            contact: "Derrickremove@example.com".into(),
            potential_value: 2000.0,
        });

        remove_lead(&mut leads, 1);
        assert!(!leads.contains_key(&1));
    }

    #[test]
    fn test_edit_lead() {
        let mut leads = HashMap::new();

        add_lead(&mut leads, 1, Lead {
            name: "Derrick".into(),
            contact: "Derrickmain@example.com".into(),
            potential_value: 4000.0,
        });

        let backup = edit_lead(&mut leads, 1, "Edited".into()).unwrap();
        assert_eq!(leads.get(&1).unwrap().name, "Edited");
        assert_eq!(backup.name, "Derrick");
    }

    #[test]
    fn test_cancel_edit() {
        let mut leads = HashMap::new();

        let original = Lead {
            name: "Derrick".into(),
            contact: "derrick@example.com".into(),
            potential_value: 3000.0,
        };

        add_lead(&mut leads, 1, original.clone());
        edit_lead(&mut leads, 1, "Changed".into());

        cancel_edit(&mut leads, 1, original.clone());
        assert_eq!(leads.get(&1).unwrap().name, "Derrick");
    }
}
