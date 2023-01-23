//RustFundamentals Assignment - Bank Account
//Max Edward | 1/22/23

use std::io; //Use is for bringing items into scope so it can be used, the std::io brings the in and out operations from the standard library.

fn main() { //This is the main function of the program, and in this case the only function in the program.

    //Variables used for bank account information, will be stored until called or until modified.
    //Let = assign variable.
    //Mut = mutable, editable. Default is unmutable, or not editable.
    let mut _account_num: i32 = 0; //Signed 32-bit integer, for 4 digit account number.
    let mut _account_pass = String::new(); //New String, for account password.
    let _account_name: String = "Max Edward".to_owned(); //String Literal, for account owner name.
    let mut _account_bal: f64 = 1337.25; //Floating point 64-bit decimal, for account balance.
    let mut _account_overdraft: bool = false; //False Boolean, for account overdraft protection enable/disable.
    let _account_stat: char = 'A'; //Character, used for account status (A = Active, D = Disabled)
    let mut _account_option = String::new(); //New String, for account option choice.

    //Getting user input for account signin
    println!("\nHello! Welcome to Bank of UAT.\n");
    println!("To proceed, please enter the last four digits of your account number followed by your password.");

    //Get input from user.
    /*
    io::stdin() is the function handle to get standard input.
    .read_line() is a method to read a line of text from the user.
    .expect is a method to handle any errors that might occur, for example, if a string is entered for an integer.
    */
    //Get user to enter Account Number.
    println!("\nAccount Number:");
    let mut _account_num_input = String::new(); //New String, for temporarily storing user input.
    io::stdin().read_line(&mut _account_num_input) //Read user input and store into _account_num_input variable.
        .expect("Invalid Account Number."); //Error message in case incorrect value is entered in prior line.
    _account_num = _account_num_input.trim().parse::<i32>() //Set _account_num variable to _account_num_input value after it is trimmed (remove whitespace) and parsed.
        .expect("Please enter a valid number for Account Number"); //Error message in case parsing has a issue.
    
    //Get user to enter Account Password.
    println!("\nPassword:");
    io::stdin().read_line(&mut _account_pass) //Read user input and store into __account_pass variable.
        .expect("Invalid Password."); //Error message in case incorrect value is entered in prior line.
    
    //After account login, welcome user.
    println!("\nWelcome back {}.\n", _account_name);
    println!("Your current account status is {}.\n", _account_stat);
    //If statement to compute correct readout of account overdraft value.
    if _account_overdraft == false{ //If _account_overdraft is set to false, say account overdraft protection is off.
        println!("Your current account overdraft protection is set to OFF.\n");   
    }
    else if _account_overdraft == true { //Else, if _account_overdraft is set to true, say account overdraft protection is on.
        println!("Your current account overdraft protection is set to ON.\n");
    }
    println!("Your account balance is ${}.\n", _account_bal);
    
    //Ask user what they would like to do, and present with option list.
    println!("What would you like to do?\n");
    println!("OPTIONS:");
    println!("Withdrawl Money");
    println!("Deposit Money");
    println!("Change Overdraft Protection");

    //Get input from user.
    let mut _account_option = String::new(); //New String, for temporarily storing user input.
    io::stdin().read_line(&mut _account_option) //Read user input and store into _account_option variable.
        .expect("Invalid Selelction"); //Error message in case incorrect value is entered in prior line.
    let _account_option = _account_option.trim(); //Trim _account_option entry from user to remove any extra whitespace not necessary, then restore in variable.

    //Withdrawl Option
    if _account_option == "Withdrawl Money" { //Read _account_option variable, if it is "Withdrawl Money", proceed.
        println!("\nHow much money would you like to withdrawl?");
        let mut _withdrawl_amount: f64 = 0.0; //New floating point 64-bit decimal, for temporarily storing user input.
        let mut _withdrawl_amount_input = String::new(); //New String, for temporarily storing user input before parse.
        io::stdin().read_line(&mut _withdrawl_amount_input) //Read user input and store into _withdrawl_amount_input variable.
            .expect("Invalid Amount Entered"); //Error message in case incorrect value is entered in prior line.
        _withdrawl_amount = _withdrawl_amount_input.trim().parse::<f64>() //Set _withdrawl_amount variable to _withdrawl_amount_input value after it is trimmed (remove whitespace) and parsed.
            .expect("Please enter a valid amount"); //Error message in case parsing has a issue.
        _account_bal -= _withdrawl_amount; //Arithmetic to subtract _withdrawl_amount from _account_bal variable.
        println!("\nThank you. Your new account balance is ${}.", _account_bal); //Read out new account balance to user before exiting.
    }

    //Deposit Option
    if _account_option == "Deposit Money" { //Read _account_option variable, if it is "Deposit Money", proceed.
        println!("\nHow much money would you like to deposit?");
        let mut _deposit_amount: f64 = 0.0; //New floating point 64-bit decimal, for temporarily storing user input.
        let mut _deposit_amount_input = String::new(); //New String, for temporarily storing user input before parse.
        io::stdin().read_line(&mut _deposit_amount_input) //Read user input and store into _deposit_amount_input variable.
            .expect("Invalid Amount Entered"); //Error message in case incorrect value is entered in prior line.
        _deposit_amount = _deposit_amount_input.trim().parse::<f64>() //Set _deposit_amount variable to _deposit_amount_input value after it is trimmed (remove whitespace) and parsed.
            .expect("Invalid Amount Entered 2"); //Error message in case parsing has a issue.
        _account_bal += _deposit_amount; //Arithmetic to add _deposit_amount to _account_bal variable.
        println!("\nThank you. Your new account balance is ${}.", _account_bal) //Read out new account balance to user before exiting.
    }

    //Change Overdraft Protection Option
    if _account_option == "Change Overdraft Protection" { 
        if _account_overdraft == false { //If _account_overdraft variable is set to false, then set it to true (flip boolean).
            _account_overdraft = true;
        }
        else if _account_overdraft == true { //If _account_overdraft variable is set to true, then set it to false (flip boolean).
            _account_overdraft = false;
        }
        if _account_overdraft == false { //If new _account_overdraft value is false, read it out as OFF.
            println!("\nAccount Overdraft Protection has been changed to OFF.");   
        }
        else if _account_overdraft == true { //If new _account_overdraft value is true, read it out as ON.
            println!("\nAccount Overdraft Protection has been changed to ON.");
        }
    }
} //End main function of program.