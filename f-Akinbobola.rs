use std::io;

fn main() {
	let mut _number_of_siblings = String::new();
	let mut _first_name_of_sibling = String::new();
	let mut _age_of_sibling = String::new();
	let mut _gender_of_sibling = String::new();
	let mut _country_of_residence_for_sibling = String::new();
	println!("We want to know about you siblings");

	println!("How many siblings do you have: ");
	io::stdin().read_line(&mut _number_of_siblings).expect("Failed to read input");
	
	for _number_of_siblings in 0..10 {
		println!("First name of sibling: ");
		io::stdin().read_line(&mut _first_name_of_sibling).expect("Failed to read input");
		println!("Age of sibling: ");
		io::stdin().read_line(&mut _age_of_sibling).expect("Failed to read input");
		
		if _age_of_sibling >=18.to_string() {
			let mut relationship_status = String::new();
			println!("Is this sibling married, single or in a relationship");
			io::stdin().read_line(&mut relationship_status).expect("Failed to read input");
			
				if relationship_status == "married" {
					let mut amount_of_children = String::new();
					println!("Do they have any Children, if so how many: ");
					io::stdin().read_line(&mut amount_of_children).expect("Failed to read input");
						if amount_of_children >= 0.to_string() {
							let mut name_of_child = String::new();
							let mut age_of_child = String::new();
							let mut school_or_daycare_of_child = String::new();
							println!("What is the name of the child: ");
							io::stdin().read_line(&mut name_of_child).expect("Failed to read input");
							println!("How old is the child: ");
							io::stdin().read_line(&mut age_of_child).expect("Failed to read input");
							println!("What School or daycare is the child: ");
							io::stdin().read_line(&mut school_or_daycare_of_child).expect("Failed to read input");
						}
			else if relationship_status == "single" {
				let mut _occupation_status = String::new();
				println!("Are the a student or employed: ");
					if _occupation_status == "student" {
						let mut university_name = String::new();
						let mut course_of_study = String::new();
						let mut year_of_study = String::new();
						let mut studying_home_or_abroad = String::new();
						println!("What is the name of the University they study; ");
						io::stdin().read_line(&mut university_name).expect("Failed to read input");
						println!("What are they studying in the University; ");
						io::stdin().read_line(&mut course_of_study).expect("Failed to read input");
						println!("What year are they in: ");
						io::stdin().read_line(&mut year_of_study).expect("Failed to read input");
						println!("Are they studying home or abroad: ");
						io::stdin().read_line(&mut studying_home_or_abroad).expect("Failed to read input");
					}
			}
				}
			}
		}
		println!("Gender of sibling: ");
		io::stdin().read_line(&mut _gender_of_sibling).expect("Failed to read input");
		println!("Country of residence of sibling: ");
		io::stdin().read_line(&mut _country_of_residence_for_sibling).expect("Failed to read input");
}


//soorry sir time elasped