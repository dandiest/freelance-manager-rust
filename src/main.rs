#[derive(Clone)]
enum WorkStatus {
    InLine,
    Working(u32),
    Concluded,
}

#[derive(Clone)]
struct Project {
    name: String,
    customer: String,
    status: WorkStatus,
}

impl Project {
    fn print_details(&self) {
        println!("Project: {} | Customer: {}", self.name, self.customer);

        match &self.status {
            WorkStatus::InLine => {
                println!("Status: You are actually in line. Please wait your turn.")
            }
            WorkStatus::Working(percent) => {
                println!("Status: We are at {}% of the work", percent)
            }
            WorkStatus::Concluded => {
                println!("Status: Work concluded")
            }
        }
    }
}

fn main() {
    let p1 = Project {
        name: "Discord Bot".to_string(),
        customer: "L. Rossi".to_string(),
        status: WorkStatus::InLine,
    };
    let mut p2 = p1.clone();
    p2.status = WorkStatus::Working(10);
    println!("Work status: 10%");
    let mut percentage = 10;

    loop {
        p2.status = WorkStatus::Working(percentage);
        p2.print_details();

        percentage += 30;

        if percentage > 100 {
            p2.status = WorkStatus::Concluded;
            println!("Work status: Concluded");
            break;
        }
    }
}
