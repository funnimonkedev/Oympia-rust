use std::io;

fn main() {
    let mut energy: i128 = 10;
    let mut max_energy: i128 = 10;
    let mut strength: i128 = 1;
    let mut money: i128 = 0;

    println!("Welcome to the gym. type 'train' to start training, 'bench' to go to bench, 'shop' to go to shop, and 'flex' to flex on weaker gym people, 'stats' to print stats");
    loop {
        println!("--input action--");
        let mut input = String::new();
        // control input
        io::stdin().read_line(&mut input)
            .expect("Failed to read line input.");
        // trim  input string and make sure it is a string.
        let input = input.trim().to_string(); 
        println!(""); 
        if input == "train" {
            println!("--input weight--");
            let mut weight = String::new();
            io::stdin().read_line(&mut weight)
                .expect("Failed to read line input.\n");
            let weight: i128 = weight.trim().parse().unwrap();
            if weight >= energy {
                println!("\n***failed to bench***\n");
            }
            if weight < energy {
                energy = energy - weight;
                strength = strength + weight;
                println!("Weight benched succesfully");
            }
        } else if input == "bench" {
            println!("\nJerald: welcome to the bench, would you like a drink, 'yes' or 'no'? (+energy, -fourth of strength)");
            let mut option = String::new();
            io::stdin().read_line(&mut option)
                .expect("Failed to read line input.");
            let option = option.trim().to_string();
                if option == "yes" {
                    println!("*you drank the drink and regained full energy, but you lost {} strength*\n",strength/4);
                    strength = strength - strength/4;
                    energy = max_energy;
                    if strength < 0 {
                        strength = 0;
                    }
                } 
            } else if input == "shop" {
                let mut option2 = String::new();
                println!("Cashier: welcome to the shop, what would you like to buy today? ('1' to add strength to max energy) \n");
                io::stdin().read_line(&mut option2)
                    .expect("Failed to read line input.");
                let option2 = option2.trim();
                if option2 == "1" {
                    max_energy = max_energy + strength / 2;
                    println!("you converted your strength into {} new max energy levels",strength/2);
                    strength = 0;
                    if strength < 0 {
                        strength = 0;
                    }
                }
            } else if input == "flex" {
                println!("***you flexed very hard***");
                energy = energy - energy/2;
                money = money + strength/10;
                println!("you gained some money: {}\nyou lost some energy: {}\n",strength/10,energy/2);
                }
                else if input == "stats" {
                    println!("current energy: {}\nmax energy: {}\nstrength: {}\nmoney: {}\n",energy,max_energy,strength,money);
                }
            }
}
