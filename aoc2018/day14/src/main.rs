/* Part 1
fn main() {
    let mut elf1_recipe = 3;
    let mut elf2_recipe = 7;
    let mut elf1_index = 0;
    let mut elf2_index = 1;
    let mut recipes = vec![3, 7];
    let requirement = 320851;

    while recipes.len() < requirement + 10 {
        elf1_recipe = recipes[elf1_index];
        elf2_recipe = recipes[elf2_index];
        let new_recipe = elf1_recipe + elf2_recipe;
        let new_recipe1 = new_recipe / 10;
        let new_recipe2 = new_recipe % 10;

        if new_recipe1 > 0 {
            recipes.push(new_recipe1);
        }
        recipes.push(new_recipe2);

        elf1_index += (1 + elf1_recipe);
        elf1_index %= recipes.len();
        elf2_index += (1 + elf2_recipe) % recipes.len();
        elf2_index %= recipes.len();
    }

    for index in requirement..recipes.len() {
        print!("{}", recipes[index]);
    }
    println!("");
}
*/

fn main() {
    let mut elf1_recipe: usize = 3;
    let mut elf2_recipe: usize = 7;
    let mut elf1_index = 0;
    let mut elf2_index = 1;
    let mut recipes = vec![3, 7];
    let requirement = vec![3, 2, 0, 8, 5, 1];

    // always only check up to the last 30 characters
    let mut start_index: i32 = 0;
    let mut end_index: i32 = 1;
    let mut steps = 0;

    // 7 because the 1 could be pushed as part of two characters
    while !recipes[(start_index as usize)..(end_index as usize)].windows(requirement.len()).any(|window| window == requirement) {
        elf1_recipe = recipes[elf1_index];
        elf2_recipe = recipes[elf2_index];
        let new_recipe = elf1_recipe + elf2_recipe;
        let new_recipe1 = new_recipe / 10;
        let new_recipe2 = new_recipe % 10;

        if new_recipe1 > 0 {
            recipes.push(new_recipe1);
        }
        recipes.push(new_recipe2);

        elf1_index += 1 + elf1_recipe;
        elf1_index %= recipes.len();
        elf2_index += (1 + elf2_recipe) % recipes.len();
        elf2_index %= recipes.len();
        end_index = recipes.len() as i32;
        start_index = end_index - 30;
        if start_index < 0 {
            start_index = 0;
        }
    }

    let mut offset = requirement.len();
    if recipes[(recipes.len() - offset)..recipes.len()] != requirement {
        // there's an additional character
        offset += 1;
    }
    println!("Recipes: {}", recipes.len() - offset);
}