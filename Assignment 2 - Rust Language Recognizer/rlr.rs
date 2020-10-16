use std::io::{self, BufRead};
use std::*;


fn main()
{
    let operations = ['+', '-']; //holds valid operations
    let variables = ['x','y','z']; //holds valid variables
    let mut tree_char_vector:Vec<char> = Vec::new(); //vector of chars that is needed to create the parse tree
    let mut should_print_tree:bool = true; //will keep track to see if parse tree should be printed

    loop 
    {
        display_bnf_grammar();

        println!("Please enter your string now: "); // prompts user for input
        let input = io::stdin(); //gets input
        
        let mut line = String::new(); //creates a string to hold the entire input
        input.lock().read_line(&mut line).unwrap();

        if line.to_string().trim() == "quit" //user enters the word quit
        {
            println!("EXITING PROGRAM. THANK YOU!"); //displays message
            std::process::exit(1); //exits the program
        }

        let split = line.split(" "); //splits the input into substrings by spaces
        let vec = split.collect::<Vec<&str>>();//collects each substring generated and places them in a vector

        if(check_begin_end(vec[0], vec[vec.len()-1])) == true //checks for the word begin as the first word and the word end as the last word
        {
            //keywords are in correct places
            let mut final_vec = vec.clone(); //clones the input(makes a new identical one) but keeps the original
            final_vec.remove(0); //removes the word "begin"
            let last_index = final_vec.len()-1; //gets the index of the input with the first word (begin) removed
            final_vec.remove(last_index);//removes the word "end", now all that is left is the substrings that will attempt to derivate
            let final_input = final_vec.clone(); //clones the input to consist of only substrings to derivate
            let mut temp_vec = final_vec.clone(); //create a new vector based on the final vector (without begin and end)
            let mut derivation_string = String::new().to_owned(); //holds substrings to derivate as one long single string
            let tree_vec:Vec<&str> = final_vec.clone();//vector needed to create the parse tree
            
            
            let mut statement_string = String::new().to_owned(); //creates a new string
            print!("program --> {}", line); //prints out what the user entered
            statement_string.push_str("        --> ");
            statement_string.push_str("begin <stmt_list> end");
            print!("{}\n", statement_string); //prints begin <stmt_list> end, always the first thing that is shown in derivation process

/*
            ATTEMPT AT PRINTING OUT APPROPRIATE LINES OF <STMT> & <STMT_LIST>
            */
            let vec_length = final_vec.len();
//            let mut current = 1;

            /*
            for i in current..vec_length
            {
                print!("{}", i);

                if(i < vec_length)
                {
                    statement_string.push_str("<stmt_list>");
                }
                else if(i == vec_length)
                {
                    statement_string.replace("<stmt_list>", "<stmt>");
                }
                
            }
            */

            if vec_length == 1
            {
                print!("        --> begin <stmt> end\n");
            }
            else if vec_length == 2
            {
                print!("        --> begin <stmt>;<stmt_list> end\n");
                print!("        --> begin <stmt>;<stmt> end\n");
            }
            else if vec_length == 3
            {
                print!("        --> begin <stmt>;<stmt_list> end\n");
                print!("        --> begin <stmt>;<stmt>;<stmt_list> end\n");
                print!("        --> begin <stmt>;<stmt>;<stmt> end\n");
            }
            else if vec_length == 4
            {
                print!("        --> begin <stmt>;<stmt_list> end\n");
                print!("        --> begin <stmt>;<stmt>;<stmt_list> end\n");
                print!("        --> begin <stmt>;<stmt>;<stmt>;<stmt_list> end\n");
                print!("        --> begin <stmt>;<stmt>;<stmt>;<stmt> end\n");
            }
            else if vec_length == 5
            {
                print!("        --> begin <stmt>;<stmt_list> end\n");
                print!("        --> begin <stmt>;<stmt>;<stmt_list> end\n");
                print!("        --> begin <stmt>;<stmt>;<stmt>;<stmt_list> end\n");
                print!("        --> begin <stmt>;<stmt>;<stmt>;<stmt>;<stmt_list> end\n");
                print!("        --> begin <stmt>;<stmt>;<stmt>;<stmt><stmt> end\n");
            }
            /*
            END ATTEMPT
            */
            
            // for loop travels down each element in the vector of strings to derivate and places them in one string
            for i in temp_vec
            {
                derivation_string.push_str(i.trim());
                derivation_string.push(' ');
            }
            derivation_string.trim();

            //for loop that travels down each character of the string from above and places each character as an element of a char vector(includes whitespaces)
            for e in derivation_string.chars()
            {
                tree_char_vector.push(e);
            }

            //for loop that removes all whitespaces from the char vector created above
            for e in tree_char_vector.clone().iter() //this loop will remove spaces within the char vector
            {
                if let Some(index) = tree_char_vector.iter().position(|&i| i == ' ')
                {
                    tree_char_vector.remove(index);
                }
            }
            
            if check_statement_list(final_vec) //checks for correct placement of semicolons
            {
                //semicolons are where they belong
                for element in (final_input).iter().rev()//start with rightmost substring (RIGHT DERIVATION)
                {
                    let mut char_vec:Vec<char> = element.chars().collect();//creates a vector from the substring (easier to index and check)
                    if let Some(index) = char_vec.iter().position(|&i| i == ';')//removes semicolon that exists in the substrings
                    {
                        char_vec.remove(index); //remove ; (leaves with just characters to check for variables and operations)
                    }

                    let length = char_vec.len(); //gets length of current vector
                    let last_index = length-1; //gets the last index of the current vector
                    println!("Current substring is: {:?}", char_vec);
                    if check_statement(element) //there is an equal sign and the character before the equal sign is not empty
                    {
                        if char_vec.len() == 5 //there are 5 characters in the substring
                        {
                            println!("        --> <var> = <expr>");
                            if operations.contains(&char_vec[last_index-1]) //the second to last character is a valid operation
                            {
                                println!("        --> <var> = <var> {} <var>",char_vec[last_index-1]);
                                if variables.contains(&char_vec[last_index]) //the last character is a valid variable
                                {
                                    println!("        --> <var> = <var> {} {}", char_vec[last_index-1], char_vec[last_index]);
                                    if variables.contains(&char_vec[last_index-2]) //the character before the operation is a valid variable
                                    {
                                        println!("        --> <var> = {} {} {}", char_vec[last_index-2], char_vec[last_index-1], char_vec[last_index]);
                                        if variables.contains(&char_vec[0]) //the first character is a valid variable
                                        {
                                            println!("        --> {} = {} {} {}", char_vec[0], char_vec[last_index-2], char_vec[last_index-1], char_vec[last_index]);
                                        }
                                        else
                                        {
                                            println!("ERROR! Invalid variable: {} in substring {}\nPlease review grammar and try again", char_vec[0], element); //states error if variable is invalid at position and string
                                            should_print_tree = false;
                                        }
                                    }
                                    else
                                    {
                                        println!("ERROR! Invalid variable: {} in substring {}\nPlease review grammar and try again", char_vec[last_index-2], element); //states error if variable is invalid at position and string
                                        should_print_tree = false;
                                    }
                                }
                                else
                                {
                                    println!("ERROR! Invalid variable: {} in substring {}\nPlease review grammar and try again", char_vec[last_index], element); //states error if variable is invalid at position and string
                                    should_print_tree = false;
                                }
                            }
                            else
                            {
                                print!("ERROR! Invalid operation: {} in substring {}\nPlease review grammar and try again", char_vec[last_index-1], element);//states error if operation is invalid at position and string
                                should_print_tree = false;
                            }
                        }
                        else if char_vec.len() == 3 //there are 3 characters in the substring (<var> op <var>)
                        {
                            println!("        --> <var> = <expr>");
                            println!("        --> <var> = <var>");
                            if variables.contains(&char_vec[char_vec.len()-1]) //the last character is a valid variable
                            {
                                println!("        --> <var> = {}", char_vec[char_vec.len()-1]);
                                if variables.contains(&char_vec[0]) //the first character is a valid variable
                                {
                                    println!("        --> {} = {}", char_vec[0], char_vec[char_vec.len()-1]);
                                }
                                else
                                {
                                    println!("ERROR! Invalid variable: {} in substring {}\nPlease review grammar and try again", char_vec[0], element); //states error if variable is invalid at position and string
                                    should_print_tree = false;
                                }
                            }
                            else
                            {
                                println!("ERROR! Invalid variable: {} in substring {}\nPlease review grammar and try again", char_vec[char_vec.len()-1], element); //states error if variable is invalid at position and string
                                should_print_tree = false;
                            }
                        }
                        else //input length is too short or too long
                        {
                            println!("ERROR! Invalid input string: {}\nPlease review grammar and try again", element);
                            should_print_tree = false;
                        }
                    }
                }

                if should_print_tree //if it is still true (no errors occured)
                {
                    println!("\n\nPRINTING PARSE TREE\n-------------------------\n");
                    display_parse_tree(tree_vec);
                }
                
            }
            else
            {
                println!("ERROR! Ensure that the semicolons are where they need to be\nPlease review grammar and try again"); //semicolons are not found where they belong, state error and prompt for input again
            }
        }
        else 
        {
            println!("Failed to attempt derivation. Please make sure the input you entered starts with \"begin\" and terminates with \"end\""); //keywords are not found where they belong, state error and prompt for input again
        }
    }

}


fn display_bnf_grammar() //function prints the BNG grammar
{
    print!("\n\n-------------------------------------------------\n");
    println!("      BNF Grammar");
    print!("\n");
    println!("   <program> --> begin <stmt_list> end");
    println!(" <stmt_list> --> <stmt>\n                 |<stmt> ; <stmt_list>");
    println!("      <stmt> --> <var> = <expression>");
    println!("       <var> --> x | y | z");
    println!("<expression> --> <var> + <var>\n                 |<var> - <var>\n                 |<var>\n-------------------------------------------------\n");
}

fn check_begin_end(first: &str, last: &str) -> bool //checks that first word is begin and last word is end
{
    if (first.trim() == "begin") & (last.trim() == "end")
    {
        return true;
    }
    false
    
}

fn check_statement_list(fstring: Vec<&str>) -> bool //checks that all substrings EXCEPT the last has a semicolon at the end
{
    let mut temp = fstring.clone();
    temp.remove(fstring.len()-1);
    let mut last = fstring.last().clone();

    for element in temp //checks that every substring ends with a semicolon except the last one
    {
        let char_vec:Vec<char> = element.chars().collect();
        let ch = char_vec[char_vec.len()-1];
        if ch != ';'
        {
            println!("A semicolon is missing at the end of statement: {}\nPlease review grammar and try again.", element);
            return false;
        }
    }

    for element in last //checks that the last substring does not end with a semicolon
    {
        let char_vec:Vec<char> = element.chars().collect();
        let ch = char_vec[char_vec.len()-1];
        if ch == ';'
        {
            println!("A semicolon should not be at the end of the last statement: {}\nPlease review grammar and try again", element);
            return false;
        }

    }
    true
}

fn check_statement(word: &str) -> bool // first character must not be empty and second character has to be an equal sign
{
    let char_vec:Vec<char> = word.chars().collect();
    if char_vec[1] != '='
    {
        println!("An equal sign is missing at the second character of the substring {}\nPlease review grammar and try again", word);
        return false;
    }

    if char_vec[0].to_string().is_empty()
    {
        println!("The first character of the substring {} is empty\nPlease review grammar and try again", word);
        return false;
    }
    true
}

fn display_parse_tree(tree_elmts: Vec<&str>) //functions prints out the parse tree
{
    let operations = ['+', '-'];
    let variables = ['x','y','z'];
    let mut final_string = String::new().to_owned();
    final_string.push_str("program --> ");
		
	let mut index = tree_elmts.len() - 1;
	println!("<program>\n  | ");
	println!("<stmt_list>\n |");
	for element in tree_elmts.iter()
	{
		let mut c_vec:Vec<char> = element.chars().collect();
		let mut last_index = c_vec.len()-1;
	
		if c_vec.len() == 5 //there are 5 characters in the substring
		{
			println!("<stmt> \n |");
			println!("<var> = <expr> \n |  \t |");

			println!(" | \t<var> {} <var> \n |     \t  |   \t  |",c_vec[last_index-1]);
			println!(" {}  \t  {} \t  {}",c_vec[0], c_vec[2], c_vec[last_index]);
		}
	}
}