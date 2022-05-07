# loop and conditional statements.
- all of these types of statements can be implemented with a simple piece of syntax.
- for example:
		
		if : 5 == 5, INT x < 5, x.$inc():
			--- execute other code here ---
			continue
		elif : y > 7:
			--- execute other code here ---
		else:
			--- execute other code here ---

- this syntax should allow us to use a single if / else statement to do all kinds of loops and conditional statements
- the first part of the statement is "if". this is followed by a colon to define the condition. the colons inside statements should have whitepace before and after.
- the ' 5 == 5 ' statement is the intialiser, the entire loop code block is ignored if it is not true. if you just want to create a loop, replace the statement with simply ' True '
- the ' INT x < 5 ' statement creates an integer variable ' x ' with a value of 0. putting INT before the variable name creates the variable, if we had not included the ' INT ' part, it would just be a condition on an existing variable. this allows us to implement "for loop" functionality by creating a variable in the statment and checking its value simultaneously. 
- the next statement is ' x.$inc() ' and is separated from the first statement by a comma and has a colon after it, showing the change of indentation level. this function simply increments x by 1, but only when this line is returned to with the continue statement, and not when x is first initialised. it can do this because x is an integer and integers will have the increment trait ( .inc() ) by default which can be called on it. floats do not have this trait.
	- if we did not implement the ( .inc() ) statement, this would simply be a while loop, as it would run until the condition it is given is no longer true. this means the we can control whether they use a for loop or while loop by simply including or not including an iterative statement.
	- we can also remove the loop functionality by removing the ' continue ' statement at the end of the code to be run. (you can also remove the condition and incrementation statement in the line) this means the code does not return to the if statement and simply continues with the code.
	- the final thing we can do is replace this condition with either ' True ' or simply ' 1 '. this will make the loop always true so it will iterate until it is exited manually
		- note: put feature in compiler that detects loops that cannot terminate and warn the user)
- finally for this part we have the continue statement. this simply tells the code to return to the start of the loop and execute all of the code after the if statement (this time including the increment statment)

- the next part of the code is an elif statement. 
- this essentially does exactly the same thing as the if statement but it only initialises if the first condition of the previous statement is not satisfied (in this case ' 5 == 5 '). the second condition in the statement has no impact on the execution of subsequent elif statements.
- in the statement ' y > 7 ', it is put between two colons with the code to be executed if true after the colons.

- else is fairly self explanatory, it simply executes if no if or elif statements before it were true.
- the loop functionality can also be used with else (and obviously elif as well)
- heres another example:

		if : x < 3, False, x :     // *
			x.$inc(4)
		elif: x < 5 :              // **
			x.$inc(2)
		else: x < 100, x.$inc(1) : continue // ***
			
		
	- *  (False, x) can be omitted as it is not necessary, its just kept in to show how the syntax is similar
	- ** in this (False, x) is already omitted.
	- *** else only has two statements after it, the first condition is the loop condition and the second is the incrementor. continue can be put on the same line after the second colon to show that no code should be executed after the loop is intialised and it should just loop the incrementation statement.